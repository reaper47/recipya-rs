### Recipya - English (US)

-brand = Recipya

## Shared vocabulary

cookbooks = Livres de recettes
recipes = Recettes
reports = Rapports
settings = Paramètres
shopping = Shopping
login = Sign in
logout = Log out
signup = Sign up
unknown = Inconnu
of = of
or = or
title = { $title }:
count = { NUMBER($count) }

## Shared table column headers

column-name = Name
column-number = Number
column-description = Description
column-category = Category
column-source = Source
column-website = Website
column-country = Country
column-setting = Paramètre
column-environment = Environment
column-value = Value
country-usa = United States of America

## Inputs

add-tool-input =
    .placeholder = 1 poêle
add-ingredient-input =
    .placeholder = 1 tasse d'oignons hachés
add-instructions-input =
    .placeholder = Mix all ingredients together
confirm-password-input =
    .label = Confirm password
    .placeholder = Retype password
email-input =
    .label = Email
    .placeholder = Enter email address
user-input =
    .label = Username or email
    .placeholder = Enter your username
new-password-input =
    .label = New password
    .placeholder = Enter new password
password-input =
    .label = Password
    .placeholder = Enter password
current-password-input =
    .label = Current password
    .placeholder = Enter current password
paper-sizes-search =
    .placeholder = Search for a paper size
search-recipe-input =
    .placeholder = Search for a recipe
searchbar-input =
    .placeholder = Search for recipes…
searchbar-no-results = No results
searchbar-action = Rechercher
recipe-rating-aria-label =
    { $count ->
        [one] { $count } star
       *[other] { $count } stars
    }

## Authentication

auth-account-verified = Your account has been verified.
auth-password-reset-requested-title = Password reset requested
auth-password-reset-requested-text = An email with instructions on how to reset your password has been sent to you.
auth-token-expired-title = Token expired
auth-token-expired-text = The token associated with the URL has expired.
forgot-password-form-title = Mot de passe oublié
reset-password-form-title = Réinitialiser le mot de passe
reset-password-form-submit = Set new password
login-form-tab-title = { login }
login-form-title = Sign in to { -brand }
login-form-greeting-1 = La cuisinière est chaude. Allons-nous cuisiner ?
login-form-create-account = Create an account
login-form-forgot-password = Mot de passe oublié ?
login-form-remember-me = Remember me
register-form-tab-title = Register
register-form-title = Create your account
register-form-greeting-1 = Your culinary journey starts here!
register-form-account-exists = Already have an account?
change-password-form-title = Changer le mot de passe
delete-account-form-title = Supprimer le compte
delete-account-form-description = Permanently delete your account
delete-account-form-confirm = Are you sure you want to delete your account? This action cannot be undone.

## Toasts: general

toast-title-success = Success
toast-title-error = L’opération a échoué
toast-title-warning = Attention
toast-export-data-failed = Unable to create export data response.
toast-http-client-failed = Impossible de créer le client HTTP.
toast-download-token-not-found = Unable to find the download token.
toast-components-failed = Error fetching components.
toast-paper-sizes-failed = Unable to fetch paper sizes.
toast-fetch-url-failed = Unable to fetch URL.
toast-invalid-url = URL invalide
toast-export-file-open-failed = Unable to open the export file.
toast-payload-empty = Payload cannot be empty.
toast-payload-invalid = Payload is invalid.
toast-read-response-failed = Impossible de lire la réponse.

## Toasts: auth

toast-auth-account-no-delete = This account cannot be deleted.
toast-auth-credentials-invalid = Credentials are invalid.
toast-auth-email-or-password-invalid = Email is invalid or passwords do not match.
toast-auth-access-token-failed = Unable to generate access token.
toast-auth-refresh-token-failed = Unable to generate a refresh token.
toast-auth-password-invalid = Password is invalid.
toast-auth-password-no-match = Passwords do not match.
toast-auth-password-updated = Your password has been updated.
toast-auth-user-password-updated = User password updated.
toast-auth-password-update-failed = Unable to update password.
toast-auth-password-schema-update-failed = Unable to update the password schema.
toast-auth-new-password-same-as-old = New password cannot be the same as the current one.
toast-auth-registration-error = An error occurred during registration.
toast-auth-verify-email = Verify your email address

## Toasts: users

toast-users-cannot-delete-admin = Cannot delete an administrator.
toast-users-delete-success = Utilisateur supprimé.
toast-users-delete-failed = Unable to delete user.
toast-users-fetch-settings-failed = Error fetching user settings.
toast-users-fetch-users-failed = Unable to fetch users.
toast-users-fetch-user-failed = Failed to fetch user.
toast-users-not-found = User not found.

## Toasts: recipes

toast-recipes-add-category-failed = Unable to add recipe category.
toast-recipes-add-collection-failed = Unable to add recipe to collection.
toast-recipes-category-empty = Category cannot be empty or uncategorized.
toast-recipes-category-delete-failed = Failed to delete recipe category.
toast-recipes-count-failed = Error fetching the number of recipes.
toast-recipes-delete-failed = Recipe could not be deleted.
toast-recipes-exist = Recipe exists.
toast-recipes-export-failed = Failed to export recipes.
toast-recipes-fetch-progress = Fetching recipes…
toast-recipes-fetch-failed = Error fetching recipes.
toast-recipes-fetch-ingredients-failed = Failed to fetch ingredients.
toast-recipes-import-status =
    Imported { $num_success ->
        [one] { NUMBER($num_success) } recipe
       *[other] { NUMBER($num_success) } recipes
    }. Skipped { NUMBER($num_skipped) }.
toast-recipes-insert-failed = Failed to insert recipe.
toast-recipes-keywords-fetch-failed = Error fetching recipe keywords.
toast-recipes-none-found = No recipes found.
toast-recipes-none-found-export = No recipes found for export.
toast-recipes-none-selected-export = No recipes selected for export.
toast-recipes-not-changed = Recipe has not changed.
toast-recipes-not-found = Recipe not found.
toast-recipes-not-found-view = The recipe you requested to view cannot be found.
toast-recipes-parse-json-failed = Error parsing recipe schema JSON.
toast-recipes-parsing-progress-start = Parsing recipes…
toast-recipes-parsing-failed = An error occurred while parsing the recipes. Please check the logs.
toast-recipes-preview-failed = Error rendering recipe preview.
toast-recipes-preparing-import = Preparing import…
toast-recipes-saving-media = Saving media
toast-recipes-saving-recipes = Saving recipes
toast-recipes-scrape-failed = Error scraping recipe or recipe source is not a URL.
toast-recipes-share-link-failed = Error creating shared recipe link.
toast-recipes-times-format-failed = Error formatting recipe times.
toast-recipes-toggle-favourites-failed = Error toggling favorite.
toast-recipes-yield-zero = Le rendement doit être supérieur à zéro.

## Toasts: timeline

toast-timeline-event-created = Timeline event created.
toast-timeline-event-create-failed = Could not create timeline event.
toast-timeline-event-not-exist = Timeline event does not exist.
toast-timeline-event-fetch-failed = Could not fetch timeline event.
toast-timeline-event-edit-failed = Failed to edit timeline event.
toast-timeline-components-fetch-failed = Unable to fetch timeline components.

## Toasts: settings

toast-settings-fetch-categories-failed = Error fetching categories.
toast-settings-fetch-languages-failed = Error fetching languages.
toast-settings-invalid-nutrition-source = Nutrition source '{ $name }' is invalid.
toast-settings-invalid-timezone = Invalid time zone.
toast-settings-invalid-theme = Theme '{ $name }' is invalid.
toast-settings-save-nutrition-source-failed = Error saving selected nutrition source.
toast-settings-update-language-failed = Error updating language.
toast-settings-update-paper-size-failed = Error updating paper size.
toast-settings-update-theme-failed = Error saving selected theme.
toast-settings-update-timezone-failed = Error updating timezone.

## Toasts: shopping

toast-shopping-add-item-failed = Unable to add the shopping list item.
toast-shopping-add-items-failed = Unable to add items for the recipe.
toast-shopping-add-items-success = Items added to shopping list.
toast-shopping-create-failed = Failed to create a new shopping list.
toast-shopping-create-label-failed = Error creating shopping list label.
toast-shopping-delete-list-failed = Failed to delete shopping list.
toast-shopping-export-failed = Failed to export shopping list.
toast-shopping-fetch-list-failed = Unable to fetch the shopping list.
toast-shopping-fetch-lists-failed = Unable to retrieve shopping lists.
toast-shopping-item-check-failed = Unable to toggle item check.
toast-shopping-item-delete-failed = Unable to delete the shopping list item.
toast-shopping-item-exists = Item already exists in the list.
toast-shopping-item-fetch-failed = Unable to retrieve the shopping list item.
toast-shopping-item-name-empty = Item name must not be empty.
toast-shopping-item-update-failed = Unable to update shopping list item.
toast-shopping-label-empty = The label name cannot be empty.
toast-shopping-label-exists = Label already exists in the shopping list.
toast-shopping-label-update-failed = Unable to update the shopping list label.
toast-shopping-list-empty = Shopping list is empty.
toast-shopping-share-list-failed = Error creating shared shopping list link.
toast-shopping-title-empty = Title cannot be empty.
toast-shopping-title-exists = Title already exists.
toast-shopping-title-update-failed = Unable to update the shopping list title.
toast-shopping-write-list-failed = Unable to save the shopping list.

## Fetching recipes

fetch-recipes-failed = Fetch failed
fetch-recipes-exists = Recipe exists
fetch-recipes-none-scraped = No recipe has been scraped.
fetch-recipes-no-valid-urls = No valid URLs found.

## Recipe list

list-recipes-figure-alt = Image of the { $name } recipe
sort-recipes-action = Trier
sort-recipes-default = Default
sort-recipes-name = Name:
sort-recipes-a-to-z = A to Z
sort-recipes-z-to-a = Z à A
sort-recipes-date-created = Date créée :
sort-recipes-new-to-old = Newest to oldest
sort-recipes-old-to-new = Oldest to newest
sort-recipes-random = Aléatoire
search-favourites-button =
    .title = View all recipes marked as favorites
    .aria-label = View all recipes marked as favorites

## Recipe page

recipe-page-title-add = Add recipe manually
recipe-page-tab-title-add = { recipe-page-title-add } | { -brand }
recipe-page-tab-title-view = { $recipe } | { -brand }
recipe-page-add-to-collection = Add recipe to collection
recipe-page-add-to-favourites = Add to favorites
recipe-page-edit-recipe = Edit recipe
recipe-page-empty-collection = Your recipe collection looks a bit empty right now.
recipe-page-collection-subtext = Why not start adding recipes by clicking the { $button } button at the top?
recipe-page-category = Category
recipe-page-delete-recipe = Delete recipe
recipe-page-delete-recipe-confirm = Are you sure you want to delete this recipe?
recipe-page-description = Description
    .placeholder = This Thai curry chicken will make you drool.
recipe-page-duplicate-recipe = Duplicate recipe
recipe-page-title-input =
    .placeholder = Recipe title
recipe-page-image-alt = Image of the recipe
recipe-page-ingredients = Ingredients
recipe-page-instructions = Instructions
recipe-page-favourite = Favorite
recipe-page-media = Media
recipe-page-media-num = Média { $num }
recipe-page-media-num-long = Image { $image_num } of the recipe
recipe-page-media-enter-url =
    .placeholder = Enter the URL of an image
recipe-page-no-description = No description
recipe-page-no-tools = No tools
recipe-page-notes = Notes
    .placeholder = Write some notes about the recipe…
recipe-page-paste-image = Paste copied image
recipe-page-cook-time = Cooking time
recipe-page-prep-time = Preparation time
recipe-page-total-time = Total time
recipe-page-print-recipe = Imprimer la recette
recipe-page-options-menu = Open recipe options menu
recipe-page-section-name = Section name
recipe-page-servings = Servings
recipe-page-servings-text =
    { $count ->
        [one] { $count } serving
       *[other] { $count } servings
    }
recipe-page-share-recipe = Share recipe
recipe-page-source = Source
recipe-page-source-data-tip = The source can be a website, the name of a cookbook, a relative or friend, a magazine, etc.
recipe-page-source-label = Source :
recipe-page-source-unknown = Source: Unknown
recipe-page-toggle-screen-lock = Toggle screen lock
recipe-page-toggle-favourite = Mark or unmark as favorite
recipe-page-tools = Outils
recipe-page-youtube-video-player = YouTube vidéolecteur
recipe-page-video-currently-processed = Video is currently being processed.
recipe-page-please-refresh-later = Please refresh the page later.
recipe-page-video-num-processed = Video #{ $num } is currently being processed.
recipe-page-start-timer = Démarrer le minuteur de { $duration }
edit-page-title = Modifier { $recipe }
edit-page-tab-title = Edit { $recipe } | { -brand }
rescrape-page-title = Rescrape { $recipe }
rescrape-page-tab-title = Rescrape { $recipe } | { -brand }
rescrape-page-no-keywords = No keywords
rescrape-page-no-images = No images
section-add = New section
keywords-new = New keyword

## Recipe timeline

recipe-timeline = Chronologie
recipe-timeline-title = Recipe timeline
recipe-timeline-add = Add event to timeline
recipe-timeline-recipe-made = Recipe made
recipe-timeline-title-label = Title
recipe-timeline-date = Date
    .placeholder = Pick a date
recipe-timeline-image = Image
recipe-timeline-comment = Comment
    .placeholder = How did your dish go today?
recipe-timeline-rating = Note
recipe-timeline-event = Timeline event
recipe-timeline-event-image = Event image
recipe-timeline-no-image = No image
recipe-timeline-open = Open timeline

## Add recipe page

add-recipe-page-title = Add recipe
add-recipe-page-tab-title = { add-recipe-page-title } | { -brand }
add-recipe-page-standard = standard
manual-recipe-card-title = Manuel
manual-recipe-card-image-alt = Writing on a piece of paper with a traditional pen.
manual-recipe-card-description = Add a new recipe by filling out its content manually.
manual-recipe-card-fill-in = Remplir
scan-card-title = Scannez
scan-card-image-alt = A cell phone used as a camera.
scan-card-description = Upload the image files or PDF of the recipe you want to add, or take a picture using your device's camera.
scan-card-dialog-title = Scan Recipe
scan-card-dialog-description = Select your recipe's images ordered by page or a recipe document in PDF format.
fetch-website-card-title = Site web
fetch-website-card-image-alt = Earth connected from end to end by telecommunications.
fetch-website-card-description = Fetch a recipe or recipes from { $supported } websites. If the website is unsupported, the software will try to extract the recipe, but there is no guarantee of success.
fetch-website-card-fetch-recipes = Fetch recipes
fetch-website-card-dialog-title = Récupérer des recettes depuis des sites web
fetch-website-card-dialog-description = Enter one or more URLs, each on a new line.
fetch-website-card-search =
    .placeholder = Rechercher sur un site web
import-apps-card-title = Importer
import-apps-card-image-alt = A group of shipping containers on a cargo ship.
import-apps-card-description = Import recipes via API from Mealie, Tandoor, and Nextcloud, as well as from { $apps }, and raw JSON files adhering to the { $schema } standard.
import-apps-card-various-apps = various apps
import-apps-card-recipe-schema = schéma de recette
import-apps-card-app = Application
import-apps-card-file-formats = Formats de fichiers
import-apps-card-search =
    .placeholder = Search for an application
import-recipes-dialog-title = Importer les recettes
import-recipes-dialog-software = Logiciel
import-recipes-dialog-app = Choose an application
    .placeholder = Pick an application
import-recipes-dialog-select-file = Select a file
import-recipes-dialog-api = API
import-recipes-dialog-choose-api = Choose an API
    .placeholder = Pick an API
import-recipes-dialog-base-url = URL de base
raw-json-dialog-beautify = Beautify
raw-json-dialog-wrap = Envelopper
raw-json-dialog-copy-example = Copier l'exemple de JSON
raw-json-dialog-paste-json = Paste JSON
raw-json-dialog-paste-json-long = Paste JSON on the left to render a preview here. For example, try:
raw-json-dialog-type-json = Prêt — Commencez à taper ou collez du JSON pour voir la coloration syntaxique
raw-json-dialog-preview = Aperçu
raw-json-dialog-fetching = Récupération…
raw-json-dialog-schema = Schéma
raw-json-dialog-fetch-schema = Fetching schema…
raw-json-dialog-fetch-wait = Please wait while the schema is being fetched…
bookmarklet-name = { -brand } Bookmarklet
    .data-tip = Simply drag this link to your bookmarks bar and click the bookmark while on a recipe website. If a recipe schema downloads successfully, you can import it here.
bookmarklet-description = Vous pouvez également télécharger directement les fichiers de schéma de recette à l’aide du { $bookmarklet }.

## Nutrition

nutrition-per-100g = Nutrition (per 100 g)
nutrition-per-serving = Nutrition (per serving)
nutrition-facts = Nutrition Facts
nutrition-calories = Calories
nutrition-serving-size = Taille de la portion
nutrition-total-carbs = Total carbohydrates
nutrition-sugars = Sucres
nutrition-protein = Protéine
nutrition-total-fat = Total fat
nutrition-sat-fat = Saturated fat
nutrition-unsat-fat = Unsaturationfett
nutrition-trans-fat = Trans fat
nutrition-cholesterol = Cholesterol
nutrition-sodium = Sodium
nutrition-fibre = Fiber
nutrition-sources-dialog-search =
    .placeholder = Search for a source
nutrition-sources-dialog-last-updated = Dernière mise à jour

## Generic actions

action-actions = Actions
action-add = Add
action-cancel = Annuler
action-clear = Effacer
action-close = Fermer
action-copy = Copier
action-delete = Supprimer
action-download = Télécharger
action-duplicate = Dupliquer
action-edit = Modifier
action-end = Terminer
action-export = Exporter
action-fetch = Récupérer
action-import = Importer
action-loading = Chargement…
action-print = Imprimer
action-rescrape = Rescrape
action-restore-original = Restore original
action-retry = Réessayer
action-retry-all = Retry all
action-see = Voir
action-share = Partager
action-submit = Soumettre
action-support = Support
action-update = Mettre à jour
action-upload = Upload
action-view = View
action-visit = Visit

## Keyboard shortcuts

shortcuts-hint = Raccourci : { $keys }
keyboard-shortcuts-title = Raccourcis clavier
keyboard-shortcuts-global = Global
keyboard-shortcuts-open-settings-dialog = Open the settings dialog
keyboard-shortcuts-create-recipe-manually = Create a new recipe manually
keyboard-shortcuts-open-import-recipes-dialog = Open the Import Recipes dialog
keyboard-shortcuts-open-fetch-recipes-dialog = Open the dialog to fetch recipes from websites
keyboard-shortcuts-open-reports-page = Ouvrir la page des rapports
keyboard-shortcuts-manual-recipe-form = Formulaire manuel de recette
keyboard-shortcuts-save-recipe = Save the recipe
keyboard-shortcuts-edit-recipe-form = Modifier le formulaire de recette
keyboard-shortcuts-view-recipe = View recipe
keyboard-shortcuts-duplicate-recipe = Duplicate the recipe
keyboard-shortcuts-edit-recipe = Modifier la recette
keyboard-shortcuts-toggle-favourite-recipe = Mark/unmark the recipe as favorite
keyboard-shortcuts-print-recipe = Imprimer la recette
keyboard-shortcuts-share-recipe = Partager la recette
keyboard-shortcuts-delete-recipe = Delete the recipe
keyboard-shortcuts-macos-replacement = { $ctrl } peut également être remplacé par { $cmd } pour les utilisateurs de macOS.

## Export and paper sizes

export-data-form = Exporter les données
    .description = Téléchargez vos données au format de fichier sélectionné.
export-data-table-favourite = Favorite
export-data-table-rating = Note
export-data-table-page = Page
paper-sizes-table-size-mm = Taille (mm)
paper-sizes-table-size-in = Size (inches)
paper-size-dimensions = { NUMBER($width, maximumFractionDigits: 2) } × { NUMBER($height, maximumFractionDigits: 2) }
print-view-title = Affichage d'impression
share-link-copy-clipboard = Copier dans le presse-papiers
simple-page-back-home = Back to Home

## Navigation

main-logo =
    .alt = Main logo
    .title = { -brand }
nav-sidebar-aria-label = Ouvrir la barre latérale du menu
nav-sidebar-new-cookbook-prompt = Enter the name of your cookbook
nav-sidebar-add-cookbook = Add cookbook
nav-sidebar-close = Close sidebar
avatar-menu-button-title = Ouvrir le menu de l'avatar
avatar-menu-update-available = Nouvelle mise à jour
avatar-menu-guide = Guide
settings-dialog-content-loading = Content is loading…
pagination-aria-label = Page { $page_num }, current page
pagination-prev-page = Previous page
pagination-next-page = Next page
pagination-goto-page = Go to page { $page_num }
pagination-summary =
    Showing { NUMBER($from) }-{ NUMBER($to) } of { NUMBER($total) } { $count ->
        [one] result
       *[other] results
    }

## Reports

reports-tab-title = { reports } | { -brand }
reports-no-entries = No reports
reports-execution-time = Temps d'exécution : { $duration }
reports-total = Total
reports-table-entity = Entité
reports-table-level = Level
reports-table-error-code = Code d'erreur
reports-table-error-reason = Raison de l'erreur
reports-table-duration = Durée
reports-table-reason = Raison : { $reason }
reports-table-duration-arg = Durée : { $d }
reports-table-code-arg = Code: { $code }

## Search help

search-help-title = Aide à la recherche
search-help-description = Le tableau suivant fournit des exemples sur la manière d'effectuer diverses recherches. Vous pouvez combiner n'importe lesquels de ces éléments dans n'importe quel ordre.
search-help-example = Example
search-help-search = Search
search-help-any-field = N'importe quel champ
search-help-any-field-category = Any field in the category
search-help-any-field-name-category = Any field, name, and category
search-help-by-name-category = By name and category
search-help-by =
    { $field ->
        [category] By category
        [cuisine] By cuisine
        [ingredient] By ingredient
        [instruction] By instruction
        [keyword] By keyword
        [source] By source
        [subcategory] By subcategory
        [tool] By tool
       *[name] By name
    }
search-help-multiple =
    { $field ->
        [categories] Multiple categories
        [cuisines] Multiple cuisines
        [ingredients] Multiple ingredients
        [instructions] Multiple instructions
        [sources] Multiple sources
        [keywords] Multiple keywords
       *[tools] Multiple tools
    }

## Sample search terms

search-help-best = best
search-help-beverages = boissons
search-help-big-green-squash = gros courge verte
search-help-biscuits = biscuits
search-help-blender = blender
search-help-breakfast = breakfast
search-help-butter = butter
search-help-chicken = chicken
search-help-chicken-kyiv = chicken kyiv
search-help-cocktails = cocktails
search-help-dinner = dinner
search-help-japanese = Japanese
search-help-lunch = lunch
search-help-mardi-gras = Mardi Gras
search-help-melt-butter = faire fondre le beurre
search-help-olive-oil = olive oil
search-help-onions = onions
search-help-preheat-oven-350 = préchauffer le four à 350
search-help-thyme = thyme
search-help-ukrainian = Ukrainian
search-help-wok = wok

## Settings

settings-tabs-general = General
settings-tabs-connections = Connexions
settings-tabs-data = Données
settings-tabs-server = Server
settings-tabs-admin = Administrateur
settings-tabs-account = Compte
settings-tabs-about = À propos
settings-admin-default-theme = Default theme
    .description = Sets the default theme for all users.
users-label = Utilisateurs
users-delete-question = Are you sure you want to delete this user? This action cannot be undone.
email-configuration-title = Email configuration
email-configuration-description = Cette connexion est configurée à l’aide de variables d’environnement.
email-configuration-host = Hôte
email-configuration-from = From
email-configuration-username = Nom d’utilisateur
email-configuration-password = Mot de passe
settings-connections-azure-ai-document-intelligence = Azure AI Document Intelligence
    .description = Cette connexion sert à numériser les images de recettes.
settings-connections-established = Connection établie
settings-connections-no-connection = No connection
settings-connections-endpoint = Point de terminaison
settings-connections-vision-placeholder = URL du point de terminaison Vision
settings-connections-resource-key = Resource key
    .placeholder = Resource key
settings-connections-test = Tester la connexion
settings-general-no-edit-runtime = Ne peut pas être modifié pendant l'exécution.
settings-general-language = Language
    .description = Choose the language for the UI.
settings-general-paper-size = Paper Size
    .description = Choose your preferred paper size for documents.
settings-general-theme = Theme
    .description = Select your preferred theme.
settings-general-themes-credits = Credits to DaisyUI for this list
settings-general-timezone = Timezone
    .description = Display times in the selected timezone.
settings-general-view-sizes = View sizes
server-configuration-table-title = Configuration
server-configuration-table-autologin = Connexion automatique
    .description = Se connecte automatiquement à l'utilisateur par défaut sans nécessiter d'identifiants.
server-configuration-table-allow-signups = Allow signups
    .description = Allows new users to create accounts.
server-configuration-table-is-demo = Is this a demo?
    .description = Enables demo mode with restricted write operations.
settings-recipes-categories = Categories
settings-recipes-new-category = New category
settings-recipes-bold-ingredients = Ingrédients en gras
    .description = Ingrédients en gras dans les instructions
settings-recipes-convert-automatically = Convert automatically
    .description = Convert new recipes to your preferred measurement system.
settings-recipes-measurement-system = Système de mesure
settings-recipes-nutrition-data-source = Source des données nutritionnelles
    .description = Choisissez la base de données nutritionnelle utilisée pour calculer les informations nutritionnelles.
settings-recipes-view-sources = Afficher les sources
placeholders-title = Placeholders

## Shopping

shopping-list-title = Shopping lists
shopping-list-tab-title = { shopping-list-title } | { -brand }
shopping-list-add-to-list = Add to shopping list
shopping-list-add-ingredients = Add ingredients to shopping list
shopping-list-add-label = Add label
shopping-list-delete-list = Delete list
shopping-list-delete-list-confirm = Are you sure you want to delete this list?
shopping-list-empty-data = No shopping data provided.
shopping-list-create-first-list = Create your first shopping list to get started.
shopping-list-for-label = Pour
shopping-list-select-a-list = Select a shopping list to view its items.
shopping-list-new-list-name = New shopping list name
shopping-list-no-label = No label
shopping-list-no-list-available = No shopping list is available.
shopping-list-no-items = The shopping list has no items.
shopping-list-text = Text
shopping-list-pick-list = Choose a shopping list
shopping-list-print-list = Imprimer la liste
shopping-list-share-list = Partager la liste
shopping-list-item-name =
    .placeholder = Sirloin steak
shopping-list-item-quantity =
    .placeholder = 500 g (optional)
shopping-list-item-notes =
    .placeholder = Notes (optional)
add-shopping-list-table-ingredient = Ingredient
add-shopping-list-table-quantity = Quantity
add-shopping-list-table-notes = Notes
add-shopping-list-table-include = Incluir

## About and updates

modes-view-label = Affichage du mode
version = { -brand } version
update-div-available = (Mise à jour disponible)
update-div-check-for-updates = Vérifier les mises à jour
update-div-checking = Vérification en cours…
update-div-latest = (dernier)
update-div-last-checked-at = Dernière vérification : { $date }
update-div-last-updated-at = Dernière mise à jour : { $date }
update-div-read-release-notes = Lisez les { $link }
update-div-release-notes = notes de version
supported = supported
