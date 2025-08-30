function initJSONHighlighter(containerId) {
    new JSONHighlighter(containerId);
}

class JSONHighlighter {
    /**
     * @param {string|HTMLElement} container - element id or the element itself
     */
    constructor(container) {
        const root = typeof container === 'string'
            ? document.getElementById(container)
            : container;

        if (!root) {
            throw new Error(`JSONHighlighter: container '${container}' not found`);
        }

        this.textarea = root.querySelector('#json-input');
        this.highlightedContent = root.querySelector("#highlighted-content");
        this.infoBar = root.querySelector("#info-bar");
        this.formatBtn = root.querySelector("#beautify");
        this.clearBtn = root.querySelector("#clear");
        this.previewOutput = root.querySelector("#preview-output");
        this.isPreviewDisabled = false;

        if (!this.textarea || !this.highlightedContent || !this.infoBar) {
            throw new Error('JSONHighlighter: required child elements missing');
        }

        this.TOKEN_RE = /("(\\u[a-fA-F0-9]{4}|\\[^u]|[^\\"])*"(\s*:)?|\btrue\b|\bfalse\b|\bnull\b|-?\d+(?:\.\d*)?(?:[eE][+\-]?\d+)?|[{}[\],])/g;

        this._timeout = null;
        this._raf = 0;
        this._lastText = '';
        this._lastParsed = {ok: false, value: null, error: null};

        this._init();
        this._highlightSchema(root);
    }

    _init() {
        this.textarea.addEventListener("scroll", () => {
            this._syncScroll(this.highlightedContent, this.textarea);
        });

        this.textarea.addEventListener("input", () => {
            this._scheduleUpdate();

            clearTimeout(this._timeout)
            this._timeout = setTimeout(() => {
                if (!this.isPreviewDisabled) {
                    this._preview();
                }
            }, 250);
        });

        this.textarea.addEventListener("paste", () => {
            requestAnimationFrame(() => {
                this._scheduleUpdate(true)
            });
        });

        if (this.formatBtn) {
            this.formatBtn.addEventListener("click", () => {
                this._formatJSON()
            });
        }

        if (this.clearBtn) {
            this.clearBtn.addEventListener("click", () => {
                this.textarea.value = "";
                this.previewOutput.innerHTML = "";
                this._scheduleUpdate(true);
            });
        }

        this._scheduleUpdate(true);
    }

    _scheduleUpdate(force = false) {
        const text = this.textarea.value;
        if (!force && text === this._lastText) {
            return;
        }

        this._lastText = text;

        if (this._raf) {
            cancelAnimationFrame(this._raf);
        }

        this._raf = requestAnimationFrame(() => {
            this._raf = 0;
            this._lastParsed = this._parse(text);

            this._renderHighlight(this.highlightedContent, text, this._lastParsed.ok);
            this._renderInfo(text, this._lastParsed);
            this._syncScroll(this.highlightedContent, this.textarea);
        });
    }

    _syncScroll(highlightedContent, textarea) {
        highlightedContent.scrollTop = textarea.scrollTop;
        highlightedContent.scrollLeft = textarea.scrollLeft;
    }

    _highlightSchema(root) {
        const content = root.querySelector("#highlighted-content2");
        const textarea = root.querySelector('#json-schema');
        this._renderHighlight(content, textarea.value, true);
        textarea.addEventListener("scroll", () => {
            this._syncScroll(content, textarea);
        });
    }

    _parse(text) {
        const trimmed = text.trim();
        if (!trimmed) {
            return {
                ok: false,
                value: null,
                error: null,
            };
        }

        try {
            const value = JSON.parse(text);
            return {
                ok: true,
                value,
                error: null,
            };
        } catch (err) {
            return {
                ok: false,
                value: null,
                error: err,
            };
        }
    }

    _renderHighlight(element, text, isValid) {
        if (!text.trim()) {
            element.innerHTML = '';
            return;
        }

        const escaped = this._escape(text);

        if (!isValid) {
            element.innerHTML = `<span class="json-error">${escaped}</span>`;
            return;
        }

        let result = '';
        let lastIndex = 0;
        const regex = this.TOKEN_RE;
        let match;
        while ((match = regex.exec(text)) !== null) {
            if (match.index > lastIndex) {
                result += this._escape(text.slice(lastIndex, match.index));
            }
            let m = match[0];
            let cls = '';
            if (m[0] === '"') {
                cls = m.endsWith(':') ? 'json-key' : 'json-string';
            } else if (m === 'true' || m === 'false') {
                cls = 'json-boolean';
            } else if (m === 'null') {
                cls = 'json-null';
            } else if (/^-?\d/.test(m)) {
                cls = 'json-number';
            } else if (/^[{}\[\]]$/.test(m)) {
                cls = 'json-bracket';
            } else {
                cls = 'json-punctuation';
            }
            result += `<span class="${cls}">${this._escape(m)}</span>`;
            lastIndex = regex.lastIndex;
        }

        if (lastIndex < text.length) {
            result += this._escape(text.slice(lastIndex));
        }
        element.innerHTML = result;
    }

    _escape(str) {
        return str
            .replace(/&/g, '&amp;')
            .replace(/</g, '&lt;')
            .replace(/>/g, '&gt;');
    }

    _renderInfo(text, parsed) {
        const empty = !text.trim();
        const setDisabled = (b) => {
            if (this.formatBtn) {
                this.formatBtn.disabled = b;
            }

            if (this.clearBtn) {
                this.clearBtn.disabled = b && empty;
            }

            if (this.isPreviewDisabled) {
                this.isPreviewDisabled = b;
            }
        };

        if (empty) {
            this.infoBar.textContent = 'Ready — Type or paste JSON to see syntax highlighting';
            setDisabled(true);
            return;
        }

        if (!parsed.ok) {
            this.infoBar.textContent = `✗ Invalid JSON: ${parsed.error?.message ?? 'Parse error'}`;
            setDisabled(true);
            return;
        }

        const lines = text.split('\n').length;
        const chars = text.length;

        const {objects, arrays} = this._countNodes(parsed.value);

        this.infoBar.textContent = `✓ Valid JSON — ${lines} lines, ${chars} chars, ${objects} objects, ${arrays} arrays`;
        setDisabled(false);
    }

    _countNodes(root) {
        let objects = 0, arrays = 0;
        const isObj = (v) => v !== null && typeof v === 'object';
        const seen = new WeakSet();
        const stack = [root];

        while (stack.length) {
            const node = stack.pop();
            if (!isObj(node) || seen.has(node)) {
                continue;
            }
            seen.add(node);

            if (Array.isArray(node)) {
                arrays++;
                for (let i = 0; i < node.length; i++) {
                    const v = node[i];
                    if (isObj(v)) {
                        stack.push(v);
                    }
                }
            } else {
                objects++;
                const vals = Object.values(node);
                for (let i = 0; i < vals.length; i++) {
                    const v = vals[i];
                    if (isObj(v)) {
                        stack.push(v);
                    }
                }
            }
        }

        return {objects, arrays};
    }

    _formatJSON() {
        const {ok, value} = this._lastParsed;
        if (!ok) {
            return;
        }

        this.textarea.value = JSON.stringify(value, null, 2);
        this._scheduleUpdate(true);
    }

    _preview() {
        htmx.ajax('POST', '/recipes/add/import/preview', {
            target: '#preview-output',
            swap: 'innerHTML',
            values: {'json-input': document.querySelector('#json-input').value}
        });
    }
}
