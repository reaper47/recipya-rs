---
title: Test
weight: 4
next: /docs/development/workflow
---

Recipya has extensive test suites to ensure the software works as expected. 
The tests are run during the [Go workflow](https://github.com/reaper47/recipya/blob/main/.github/workflows/go.yml)
when you open a pull request against the main branch.

Execute the following when you wish to run all the tests locally.

```bash
task test-no-build
```

## Writing Tests

It is of vital important to write tests when submitting pull requests. This 
[article](https://www.codemag.com/Article/1901071/10-Reasons-Why-Unit-Testing-Matters) explains why 
unit testing matters.

You will see many files under the `internal` folder that finish with `*_test.go`. That is where tests are written.
Please refer to the [development workflow](/guide/docs/development/workflow) section for more information and examples.

Please feel free to add as many tests as you deem fit to any of those files.
