### Recipya - English (US)

-brand = Recipya

## Shared vocabulary

cookbooks = Cookbooks
recipes = Recipes
reports = Reports
settings = Settings
shopping = Shopping
login = Log in
logout = Log out
signup = Sign up
unknown = Unknown
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
column-setting = Setting
column-environment = Environment
column-value = Value

country-usa = United States of America

## Inputs

add-tool-input =
    .placeholder = 1 frying pan

add-ingredient-input =
    .placeholder = 1 cup of chopped onions

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
    .placeholder = Search a paper size

search-recipe-input =
    .placeholder = Search a recipe

searchbar-input =
    .placeholder = Search for recipes…

searchbar-no-results = No results
searchbar-action = Search

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
auth-token-expired-text = The token associated with the URL expired.

forgot-password-form-title = Forgot password
reset-password-form-title = Reset password
reset-password-form-submit = Set new password

login-form-tab-title = { login }
login-form-title = Log in to { -brand }
login-form-greeting-1 = The stove is hot. Shall we cook?
login-form-create-account = Create an account
login-form-forgot-password = Forgot your password?
login-form-remember-me = Remember me

register-form-tab-title = Register
register-form-title = Create your account
register-form-greeting-1 = Your culinary journey starts here!
register-form-account-exists = Already have an account?

change-password-form-title = Change password

delete-account-form-title = Delete account
delete-account-form-description = Delete your account permanently
delete-account-form-confirm = Are you sure you want to delete your account? This action is irreversible.

## Toasts: general

toast-title-success = Success
toast-title-error = Operation failed
toast-title-warning = Attention

toast-export-data-failed = Failed to create export data response.
toast-http-client-failed = Could not create HTTP client.
toast-download-token-not-found = Failed to find download token.
toast-components-failed = Error fetching components.
toast-paper-sizes-failed = Failed to fetch paper sizes.
toast-fetch-url-failed = Could not fetch URL.
toast-invalid-url = Invalid URL
toast-export-file-open-failed = Failed to open export file.
toast-payload-empty = Payload cannot be empty.
toast-payload-invalid = Payload is invalid.
toast-read-response-failed = Could not read response.

## Toasts: auth

toast-auth-account-no-delete = This account cannot be deleted.
toast-auth-credentials-invalid = Credentials are invalid.
toast-auth-email-or-password-invalid = Email is invalid or passwords do not match.
toast-auth-access-token-failed = Failed to generate access token.
toast-auth-refresh-token-failed = Failed to generate refresh token.
toast-auth-password-invalid = Password is invalid.
toast-auth-password-no-match = Passwords do not match.
toast-auth-password-updated = Your password has been updated.
toast-auth-user-password-updated = User password updated.
toast-auth-password-update-failed = Failed to update password.
toast-auth-password-schema-update-failed = Failed to update password schema.
toast-auth-new-password-same-as-old = New password cannot be the same as the current.
toast-auth-registration-error = An error occurred during registration.
toast-auth-verify-email = Verify your email address

## Toasts: users

toast-users-cannot-delete-admin = Cannot delete an admin.
toast-users-delete-success = User deleted.
toast-users-delete-failed = Failed to delete user.
toast-users-fetch-settings-failed = Error fetching user settings.
toast-users-fetch-users-failed = Failed to fetch users.
toast-users-fetch-user-failed = Failed to fetch user.
toast-users-not-found = User not found.

## Toasts: recipes

toast-recipes-add-category-failed = Failed to add recipe category.
toast-recipes-add-collection-failed = Failed to add recipe to collection.
toast-recipes-category-empty = Category cannot be empty or uncategorized.
toast-recipes-category-delete-failed = Failed to delete recipe category.
toast-recipes-count-failed = Error fetching number of recipes.
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
toast-recipes-not-found-view = The recipe you requested to view is not found.
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
toast-recipes-yield-zero = Yield must be greater than zero.

## Toasts: timeline

toast-timeline-event-created = Timeline event created.
toast-timeline-event-create-failed = Could not create timeline event.
toast-timeline-event-not-exist = Timeline event does not exist.
toast-timeline-event-fetch-failed = Could not fetch timeline event.
toast-timeline-event-edit-failed = Failed to edit timeline event.
toast-timeline-components-fetch-failed = Failed to fetch timeline components.

## Toasts: settings

toast-settings-fetch-categories-failed = Error fetching categories.
toast-settings-fetch-languages-failed = Error fetching languages.
toast-settings-invalid-nutrition-source = Nutrition source '{ $name }' is invalid.
toast-settings-invalid-timezone = Invalid timezone.
toast-settings-invalid-theme = Theme '{ $name }' is invalid.
toast-settings-save-nutrition-source-failed = Error saving selected nutrition source.
toast-settings-update-language-failed = Error updating language.
toast-settings-update-paper-size-failed = Error updating paper size.
toast-settings-update-theme-failed = Error saving selected theme.
toast-settings-update-timezone-failed = Error updating timezone.

## Toasts: shopping

toast-shopping-add-item-failed = Failed to add shopping list item.
toast-shopping-add-items-failed = Failed to add items for recipe.
toast-shopping-add-items-success = Items added to shopping list.
toast-shopping-create-failed = Failed to create new shopping list.
toast-shopping-create-label-failed = Error creating shopping list label.
toast-shopping-delete-list-failed = Failed to delete shopping list.
toast-shopping-export-failed = Failed to export shopping list.
toast-shopping-fetch-list-failed = Failed to fetch shopping list.
toast-shopping-fetch-lists-failed = Failed to get shopping lists.
toast-shopping-item-check-failed = Failed to toggle item check.
toast-shopping-item-delete-failed = Failed to delete shopping list item.
toast-shopping-item-exists = Item already exists in the label.
toast-shopping-item-fetch-failed = Failed to get shopping list item.
toast-shopping-item-name-empty = Item name must not be empty.
toast-shopping-item-update-failed = Failed to update shopping list item.
toast-shopping-label-empty = Label name cannot be empty.
toast-shopping-label-exists = Label already exists in the shopping list.
toast-shopping-label-update-failed = Failed to update shopping list label.
toast-shopping-list-empty = Shopping list is empty.
toast-shopping-share-list-failed = Error creating shared shopping list link.
toast-shopping-title-empty = Title cannot be empty.
toast-shopping-title-exists = Title already exists.
toast-shopping-title-update-failed = Failed to update shopping list title.
toast-shopping-write-list-failed = Failed to write shopping list.

## Fetching recipes

fetch-recipes-failed = Fetch failure
fetch-recipes-exists = Recipe exists
fetch-recipes-none-scraped = No recipe has been scraped.
fetch-recipes-no-valid-urls = No valid URLs found.

## Recipe list

list-recipes-figure-alt = Image of the { $name } recipe

sort-recipes-action = Sort
sort-recipes-default = Default
sort-recipes-name = Name:
sort-recipes-a-to-z = A to Z
sort-recipes-z-to-a = Z to A
sort-recipes-date-created = Date created:
sort-recipes-new-to-old = Newest to oldest
sort-recipes-old-to-new = Oldest to newest
sort-recipes-random = Random

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
recipe-page-empty-collection = Your recipe collection looks a bit empty at the moment.
recipe-page-collection-subtext = Why not start adding recipes by clicking the { $button } button at the top?
recipe-page-category = Category
recipe-page-delete-recipe = Delete recipe
recipe-page-delete-recipe-confirm = Are you sure you wish to delete this recipe?
recipe-page-description = Description
    .placeholder = This Thai curry chicken will make you drool.
recipe-page-duplicate-recipe = Duplicate recipe
recipe-page-title-input =
    .placeholder = Title of the recipe
recipe-page-image-alt = Image of the recipe
recipe-page-ingredients = Ingredients
recipe-page-instructions = Instructions
recipe-page-favourite = Favorite
recipe-page-media = Media
recipe-page-media-num = Media { $num }
recipe-page-media-num-long = Image { $image_num } of the recipe
recipe-page-media-enter-url =
    .placeholder = Enter the URL of an image
recipe-page-no-description = No description
recipe-page-no-tools = No tools
recipe-page-notes = Notes
    .placeholder = Write some notes about the recipe…
recipe-page-paste-image = Paste copied image
recipe-page-cook-time = Cook time
recipe-page-prep-time = Prep time
recipe-page-total-time = Total time
recipe-page-print-recipe = Print recipe
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
recipe-page-source-data-tip = The source can be a website, name of a cookbook, a relative or friend, a magazine, etc.
recipe-page-source-label = Source:
recipe-page-source-unknown = Source: Unknown
recipe-page-toggle-screen-lock = Toggle screen lock
recipe-page-toggle-favourite = Mark or unmark as favorite
recipe-page-tools = Tools
recipe-page-youtube-video-player = YouTube video player
recipe-page-video-currently-processed = Video is currently being processed.
recipe-page-please-refresh-later = Please refresh the page later.
recipe-page-video-num-processed = Video #{ $num } is currently being processed.
recipe-page-start-timer = Start { $duration } timer

edit-page-title = Edit { $recipe }
edit-page-tab-title = Edit { $recipe } | { -brand }

rescrape-page-title = Rescrape { $recipe }
rescrape-page-tab-title = Rescrape { $recipe } | { -brand }
rescrape-page-no-keywords = No keywords
rescrape-page-no-images = No images

section-add = New section
keywords-new = New keyword

## Recipe timeline

recipe-timeline = Timeline
recipe-timeline-title = Recipe timeline
recipe-timeline-add = Add event to timeline
recipe-timeline-recipe-made = Recipe made
recipe-timeline-title-label = Title
recipe-timeline-date = Date
    .placeholder = Pick a date
recipe-timeline-image = Image
recipe-timeline-comment = Comment
    .placeholder = How did your dish go today?
recipe-timeline-rating = Rating
recipe-timeline-event = Timeline event
recipe-timeline-event-image = Event image
recipe-timeline-no-image = No image
recipe-timeline-open = Open timeline

## Add recipe page

add-recipe-page-title = Add recipe
add-recipe-page-tab-title = { add-recipe-page-title } | { -brand }
add-recipe-page-standard = standard

manual-recipe-card-title = Manual
manual-recipe-card-image-alt = Writing on a piece of paper with a traditional pen.
manual-recipe-card-description = Add a new recipe by filling out its content manually.
manual-recipe-card-fill-in = Fill in

scan-card-title = Scan
scan-card-image-alt = A cell phone used as a camera.
scan-card-description = Upload the image files or PDF of the recipe you want to add or take a picture using your device's camera.
scan-card-dialog-title = Scan recipe
scan-card-dialog-description = Select your recipe's images ordered by page or a recipe document in the PDF format.

fetch-website-card-title = Website
fetch-website-card-image-alt = Earth connected from end-to-end by telecommunications.
fetch-website-card-description = Fetch a recipe or recipes from { $supported } websites. If the website is unsupported, the software will try to extract the recipe, but there is no guarantee of success.
fetch-website-card-fetch-recipes = Fetch recipes
fetch-website-card-dialog-title = Fetch recipes from websites
fetch-website-card-dialog-description = Enter one or more URLs, each on a new line.
fetch-website-card-search =
    .placeholder = Search a website

import-apps-card-title = Import
import-apps-card-image-alt = A bunch of shipping containers on a cargo boat.
import-apps-card-description = Import recipes via API from Mealie, Tandoor, and Nextcloud, as well as from { $apps }, and raw JSON files adhering to the { $schema } standard.
import-apps-card-various-apps = various apps
import-apps-card-recipe-schema = recipe schema
import-apps-card-app = Application
import-apps-card-file-formats = File formats
import-apps-card-search =
    .placeholder = Search an application

import-recipes-dialog-title = Import Recipes
import-recipes-dialog-software = Software
import-recipes-dialog-app = Choose an application
    .placeholder = Pick an application
import-recipes-dialog-select-file = Select a file
import-recipes-dialog-api = API
import-recipes-dialog-choose-api = Choose an API
    .placeholder = Pick an API
import-recipes-dialog-base-url = Base URL

raw-json-dialog-beautify = Beautify
raw-json-dialog-wrap = Wrap
raw-json-dialog-copy-example = Copy example JSON
raw-json-dialog-paste-json = Paste JSON
raw-json-dialog-paste-json-long = Paste JSON on the left to render a preview here. For example, try:
raw-json-dialog-type-json = Ready - Start typing or paste JSON to see syntax highlighting
raw-json-dialog-preview = Preview
raw-json-dialog-fetching = Fetching…
raw-json-dialog-schema = Schema
raw-json-dialog-fetch-schema = Fetching schema…
raw-json-dialog-fetch-wait = Please wait while schema is being fetched…

bookmarklet-name = { -brand } Bookmarklet
    .data-tip = Simply drag this link to your bookmarks bar, and click the bookmark while on a recipe website. If a recipe schema downloads successfully, you can import it here.
bookmarklet-description = You may also download recipe schema files directly using the { $bookmarklet }.

## Nutrition

nutrition-per-100g = Nutrition (per 100g)
nutrition-per-serving = Nutrition (per serving)
nutrition-facts = Nutrition Facts
nutrition-calories = Calories
nutrition-serving-size = Serving size
nutrition-total-carbs = Total carbs
nutrition-sugars = Sugars
nutrition-protein = Protein
nutrition-total-fat = Total fat
nutrition-sat-fat = Saturated fat
nutrition-unsat-fat = Unsaturated fat
nutrition-trans-fat = Trans fat
nutrition-cholesterol = Cholesterol
nutrition-sodium = Sodium
nutrition-fibre = Fiber

nutrition-sources-dialog-search =
    .placeholder = Search a source
nutrition-sources-dialog-last-updated = Last updated

## Generic actions

action-actions = Actions
action-add = Add
action-cancel = Cancel
action-clear = Clear
action-close = Close
action-copy = Copy
action-delete = Delete
action-download = Download
action-duplicate = Duplicate
action-edit = Edit
action-end = End
action-export = Export
action-fetch = Fetch
action-import = Import
action-loading = Loading…
action-print = Print
action-rescrape = Rescrape
action-restore-original = Restore original
action-retry = Retry
action-retry-all = Retry all
action-see = See
action-share = Share
action-submit = Submit
action-support = Support
action-update = Update
action-upload = Upload
action-view = View
action-visit = Visit

## Keyboard shortcuts

shortcuts-hint = Shortcut: { $keys }

keyboard-shortcuts-title = Keyboard shortcuts
keyboard-shortcuts-global = Global
keyboard-shortcuts-open-settings-dialog = Open the settings dialog
keyboard-shortcuts-create-recipe-manually = Create a new recipe manually
keyboard-shortcuts-open-import-recipes-dialog = Open the import recipes dialog
keyboard-shortcuts-open-fetch-recipes-dialog = Open the fetch recipes from websites dialog
keyboard-shortcuts-open-reports-page = Open the reports page
keyboard-shortcuts-manual-recipe-form = Manual recipe form
keyboard-shortcuts-save-recipe = Save the recipe
keyboard-shortcuts-edit-recipe-form = Edit recipe form
keyboard-shortcuts-view-recipe = View recipe
keyboard-shortcuts-duplicate-recipe = Duplicate the recipe
keyboard-shortcuts-edit-recipe = Edit the recipe
keyboard-shortcuts-toggle-favourite-recipe = Mark/unmark the recipe as favorite
keyboard-shortcuts-print-recipe = Print the recipe
keyboard-shortcuts-share-recipe = Share the recipe
keyboard-shortcuts-delete-recipe = Delete the recipe
keyboard-shortcuts-macos-replacement = { $ctrl } can also be replaced with { $cmd } instead for macOS users.

## Export and paper sizes

export-data-form = Export data
    .description = Download your data in the selected file format.

export-data-table-favourite = Favorite
export-data-table-rating = Rating
export-data-table-page = Page

paper-sizes-table-size-mm = Size (mm)
paper-sizes-table-size-in = Size (inches)

paper-size-dimensions = { NUMBER($width, maximumFractionDigits: 2) } × { NUMBER($height, maximumFractionDigits: 2) }

print-view-title = Print view
share-link-copy-clipboard = Copy to clipboard
simple-page-back-home = Back to home

## Navigation

main-logo =
    .alt = Main logo
    .title = { -brand }

nav-sidebar-aria-label = Open menu sidebar
nav-sidebar-new-cookbook-prompt = Enter the name of your cookbook
nav-sidebar-add-cookbook = Add cookbook
nav-sidebar-close = Close sidebar

avatar-menu-button-title = Open avatar menu
avatar-menu-update-available = New update
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
reports-execution-time = Execution time: { $duration }
reports-total = Total

reports-table-entity = Entity
reports-table-level = Level
reports-table-error-code = Error code
reports-table-error-reason = Error reason
reports-table-duration = Duration
reports-table-reason = Reason: { $reason }
reports-table-duration-arg = Duration: { $d }
reports-table-code-arg = Code: { $code }

## Search help

search-help-title = Search help
search-help-description = The following table provides examples of how to perform various searches. You may combine any of these in any order.
search-help-example = Example
search-help-search = Search
search-help-any-field = Any field
search-help-any-field-category = Any field of category
search-help-any-field-name-category = Any field, name and category
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
search-help-beverages = beverages
search-help-big-green-squash = big green squash
search-help-biscuits = biscuits
search-help-blender = blender
search-help-breakfast = breakfast
search-help-butter = butter
search-help-chicken = chicken
search-help-chicken-kyiv = chicken kyiv
search-help-cocktails = cocktails
search-help-dinner = dinner
search-help-japanese = japanese
search-help-lunch = lunch
search-help-mardi-gras = mardi gras
search-help-melt-butter = melt butter
search-help-olive-oil = olive oil
search-help-onions = onions
search-help-preheat-oven-350 = preheat oven 350
search-help-thyme = thyme
search-help-ukrainian = ukrainian
search-help-wok = wok

## Settings

settings-tabs-general = General
settings-tabs-connections = Connections
settings-tabs-data = Data
settings-tabs-server = Server
settings-tabs-admin = Admin
settings-tabs-account = Account
settings-tabs-about = About

settings-admin-default-theme = Default theme
    .description = Sets the default theme for all users.

users-label = Users
users-delete-question = Are you sure you want to delete this user? This action is irreversible.

email-configuration-title = Email configuration
email-configuration-description = This connection is set up using environment variables.
email-configuration-host = Host
email-configuration-from = From
email-configuration-username = Username
email-configuration-password = Password

settings-connections-azure-ai-document-intelligence = Azure AI Document Intelligence
    .description = This connection is used to digitize recipe images.
settings-connections-established = Connection established
settings-connections-no-connection = No connection
settings-connections-endpoint = Endpoint
settings-connections-vision-placeholder = Vision endpoint URL
settings-connections-resource-key = Resource key
    .placeholder = Resource key
settings-connections-test = Test connection

settings-general-no-edit-runtime = Cannot be edited at runtime.
settings-general-language = Language
    .description = Choose the language for the UI.
settings-general-paper-size = Paper size
    .description = Choose your preferred paper size for documents.
settings-general-theme = Theme
    .description = Select your preferred theme.
settings-general-themes-credits = Credits to DaisyUI for this list
settings-general-timezone = Timezone
    .description = Display times in the selected timezone.
settings-general-view-sizes = View sizes

server-configuration-table-title = Configuration
server-configuration-table-autologin = Autologin
    .description = Automatically logs in the default user without credentials.
server-configuration-table-allow-signups = Allow signups
    .description = Allows new users to create accounts.
server-configuration-table-is-demo = Is demo?
    .description = Enables demo mode with restricted write operations.

settings-recipes-categories = Categories
settings-recipes-new-category = New category
settings-recipes-bold-ingredients = Bold ingredients
    .description = Bold ingredients in instructions
settings-recipes-convert-automatically = Convert automatically
    .description = Convert new recipes to your preferred measurement system.
settings-recipes-measurement-system = Measurement system
settings-recipes-nutrition-data-source = Nutrition data source
    .description = Choose the nutrition database used to calculate facts.
settings-recipes-view-sources = View sources

placeholders-title = Placeholders

## Shopping

shopping-list-title = Shopping lists
shopping-list-tab-title = { shopping-list-title } | { -brand }
shopping-list-add-to-list = Add to shopping list
shopping-list-add-ingredients = Add ingredients to shopping list
shopping-list-add-label = Add label
shopping-list-delete-list = Delete list
shopping-list-delete-list-confirm = Are you sure you wish to delete this list?
shopping-list-empty-data = No shopping data provided.
shopping-list-create-first-list = Create your first shopping list to get started.
shopping-list-for-label = For
shopping-list-select-a-list = Select a shopping list to view its items.
shopping-list-new-list-name = New shopping list name
shopping-list-no-label = No label
shopping-list-no-list-available = No shopping list available.
shopping-list-no-items = Shopping list has no items.
shopping-list-text = Text
shopping-list-pick-list = Pick a shopping list
shopping-list-print-list = Print list
shopping-list-share-list = Share list
shopping-list-item-name =
    .placeholder = Sirloin steak
shopping-list-item-quantity =
    .placeholder = 500g (optional)
shopping-list-item-notes =
    .placeholder = Notes (optional)

add-shopping-list-table-ingredient = Ingredient
add-shopping-list-table-quantity = Quantity
add-shopping-list-table-notes = Notes
add-shopping-list-table-include = Include

## About and updates

modes-view-label = View mode
version = { -brand } version

update-div-available = (update available)
update-div-check-for-updates = Check for updates
update-div-checking = Checking…
update-div-latest = (latest)
update-div-last-checked-at = Last checked: { $date }
update-div-last-updated-at = Last updated: { $date }
update-div-read-release-notes = Read the { $link }
update-div-release-notes = release notes
