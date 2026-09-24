# Roadmap

This document gives you a rough idea of the work planned ahead for Recipya. The software's current version is `nightly`. There will be no 0.X.Y release.

## v1.0.0

### Features

#### Recipes

- [ ] Add structured data to /recipes/{id}
- [ ] Implement PDF file type
- [ ] Implement OCR

#### Cookbook

- [ ] Implement cookbooks

#### Shopping Lists

- [ ] Export grocery list by selecting multiple recipes

#### Language 

- [ ] Implement i18n

### Maintenance

- [ ] Move function validations to the type system
- [ ] Update htmx to version 4.0
- [ ] Add benchmarks
- [ ] Look into improving error handling
- [ ] Prevent importing from multiple sources at the same time
- [ ] Accept importing multiple files for the same application

### Improvements

- [ ] Improve bold ingredients in instructions algorithm

## v1.1.0

### Features

#### Authentication

- [ ] LDAP/OIDC integration

#### Recipes

- [ ] Develop the scraper
- [ ] Add option to scale entire recipe
- [ ] Extract multiple images when adding recipe via URL
- [ ] Add tags & icons for special diets

#### Shopping List

- [ ] Create contacts
- [ ] Support sending SMS
- [ ] Implement upload to apps

### Maintenance

- [ ] Don't allow changing of units of measurement in recipes when changing servings
- [ ] Add queueing/retry for website import

## v1.2.0

### Features

#### Artificial Intelligence

- [ ] Integrate Ollama API
- [ ] Integrate ComfyUI to generate default images

#### Recipes 

- [ ] Nutrition breakdown
- [ ] Import recipes from video URL
- [ ] Add conversion to weight
- [ ] Dynamic measurement conversion
- [ ] Possibility to add images to the instructions

#### Other 

- [ ] Support unraid

## v1.3.0

### Features 

#### API

- [ ] Create the Recipya REST API

#### Recipes 

- [ ] Support more nutritional databases
- [ ] Share recipes across accounts
- [ ] Offer finer control over the measurement system
- [ ] Offer ingredient substitutes

#### Meal Planning 

- [ ] Implement the meal planner

## v1.4.0

### Features 

- [ ] Make Windows Installer
