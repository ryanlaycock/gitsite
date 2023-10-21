# github-website-runner

Project name a WIP!

## 🎯 Project Goals

This project aims to:

1. Create a _painless_ way to run a website through a single docker command - _on any platform_

    - Raspberry Pi in your cupboard, Digital Ocean, GKE etc...

2. Provide a simple CMS through the use of GitHub markdown files

    - You don't _need_ to write web content as HTML files 🤢, but the (personally) much simpler to write Markdown files.
    - If you want to add more content, you don't need to rebuild the server, or deploy the updated files. Simply commit to the 
    referenced GitHub repository and the site will reload.

3. Create a flexible template to completely customise the style of your website

    - We don't want to hold you back creatively, so you can write as much HTML/JS/CSS as you can tolerate, and we'll simply
    serve them! These files follow the same rules as above, so you can completely change the style of your site by simply committing
    to a GitHub repo! 🚀

## ❓ How

We do this by providing the backend server (the Rust service in this repo) as a docker image that can be run anywhere - 
and which references a GitHub project that hosts the `config.yaml` file.

This file then contains references to either and any custom HTML/CSS/JS you desire to be on your website, or markdown files from 
any publicly (or private given you provide a GitHub token) accessible GitHub repository.

See the /example-website for an example and guide to setting your own site up!

## 📋 Todo List (constantly changing)

- [ ] Pull in GitHub files as HTML
- [ ] Fetch all files on reload, and on request, storing to a local /tmp/ place
- [ ] Configure reload practise
  - On page load (testing only)
  - On page load with caching (prod advised, ensure we don't hit GitHub rate limits)
  - On call to an endpoint (manual)
- [ ] Create example website in format of my desired personal website
  - [ ] CSS/JS for standard web functions
- [ ] Dockerise the runner
- [ ] Write install docs

## References

- Templating:
  - https://crates.io/crates/handlebars
  - https://github.com/sunng87/handlebars-rust/blob/master/examples/render/template.hbs
  - https://handlebarsjs.com/guide/#block-helpers