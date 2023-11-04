# Quick Start Guide

In this short tutorial we'll show you how to run _this_ site locally, and explain some of the different concepts used to create the site.

> 💡 By the end of this tutorial you should have this site running and a grasp on editing it and adding more content

## Deploying

> #### ⚙️ Prerequisites
> This guide assumes you are trying to deploy to a linux environment, that has `curl` installed, and you have `sudo` access.

1. Create a GitHub access token

    a. You can follow this great guide [here](https://docs.github.com/en/enterprise-server@3.6/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-personal-access-token).
    > ⚠️ **This token needs no special access for this example. Do not add any roles.**

2. Download the [`install-http.sh`](https://github.com/ryanlaycock/gitsite/blob/main/example-website/install-http.sh) file:
    ```
    curl -L https://raw.githubusercontent.com/ryanlaycock/gitsite/master/example-website/install-http.sh -o install-http.sh
    ```

3. Run the installed shell script, specifying the previous GitHub access token in the command.
    ```
    bash install-http.sh <GITHUB_ACCESS_TOKEN>
    ```

4. Success! 🚀

## What is each file?

```
example-website
    |- about
    |- docs
        |- quick-start.md
```