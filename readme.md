# MRML

[![CircleCI](https://circleci.com/gh/jdrouet/mrml.svg?style=shield)](https://app.circleci.com/pipelines/github/jdrouet/mrml)
[![codecov](https://codecov.io/gh/jdrouet/mrml/branch/master/graph/badge.svg?token=L3LKpV3RpR)](https://codecov.io/gh/jdrouet/mrml)

## TODO

- Testing
  - [ ] compare properly the generated HTML
    - [x] not take in account empty class/style attributes
    - [ ] not care about orders of attributes
- CI
  - [ ] add code coverage
  - [ ] automatic deploy to crates.io
- Core
  - [x] expose the `to_html` method
  - [ ] add options to minify/not minify
  - [ ] clean by removing consecutive conditions
- components
  - [ ] mjml
    - [x] without attributes
    - [ ] with owa
    - [ ] with lang
  - [x] mj-head
  - [ ] mj-attributes
  - [ ] mj-breakpoint
  - [ ] mj-font
  - [ ] mj-preview
  - [ ] mj-style
  - [ ] mj-title
  - [x] mj-body
    - [x] without attributes
    - [x] with background-color
    - [x] with css-class
    - [x] with width
  - [ ] mj-accordion
  - [ ] mj-button
  - [ ] mj-carousel
  - [x] mj-column
    - [x] without attributes
    - [x] with background-color
    - [x] with border, border-(top|right|bottom|left)
    - [x] with border-radius
    - [x] with css-class
    - [x] with padding, padding-(top|right|bottom|left)
    - [x] with vertical-align
    - [x] with width (default: (100 / number of non-raw elements in section)%)
  - [ ] mj-divider
  - [ ] mj-group
  - [ ] mj-hero
  - [ ] mj-image
  - [ ] mj-navbar
  - [ ] mj-raw
  - [x] mj-section
    - [x] without attributes
    - [x] with background-color
    - [x] with background-repeat (default: repeat)
    - [x] with background-size (default: auto)
    - [x] with background-url
    - [x] with border, border-(top|right|bottom|left)
    - [x] with border-radius
    - [x] with css-class
    - [x] with direction (default: ltr)
    - [x] with full-width
    - [x] with padding (default: 20px 0)
    - [x] with padding-(top|right|bottom|left)
    - [x] with text-align
  - [ ] mj-social
  - [ ] mj-spacer
  - [ ] mj-table
  - [ ] mj-text
    - [x] without attributes
    - [x] mjml documentation example
    - [x] with color (default: #000000)
    - [x] with font-family (default: Ubuntu, Helvetica, Arial, sans-serif)
    - [x] with font-size (default: 13px)
    - [x] with font-style
    - [x] with font-weight
    - [x] with line-height (default: 1)
    - [ ] with letter-spacing (default: none)
    - [ ] with height
    - [ ] with text-decoration
    - [ ] with text-transform
    - [ ] with align (default: left)
    - [ ] with container-background-color
    - [ ] with padding (default: 10px 25px), padding-(top|right|bottom|left)
    - [ ] with css-class
  - [ ] mj-wrapper
