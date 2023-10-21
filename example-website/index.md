# What is GitSite?

GitSite has three main aims:

1. Make creating static websites simple and fun
    * Create base templates in HTML for the structure of your pages
    * As much or little CSS and JavaScript can be written to fully customise your site
    * Using standard templating we populate the templates based on content written in Markdown

2. Make deploying and maintaining websites a one time and simple task
    * We believe once your website is deployed, you shouldn't ever need to SSH in for updates
    * As the GitSite runner is dockerised, you can run your website anywhere, from k8s, VMs or a RaspberryPi on your desk

3. Adding more content is pain free
    * Writing content for your website should be _just_ that. That's why we use the much simpler to write Markdown
    * Simply commit this to a (private if you desire) GitHub repo and GitSite will smartly pull the latest changes, while staying within GitHub's rate limiting restrictions!
    * You don't need to write content in HTML or upload it directly to your website!