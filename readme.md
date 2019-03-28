# Coloroo (Pre-Interview Test)

## Overview

This app is a color palette sharing app for the pre-interview test.


### Features

 - Define a color palette
 - Share a palette through a url
 - *Hello World!* endpoint
 - Metadata is available throught the `/metadata` endpoint
 - Health checks exposed through the `/health` endpoint


## Getting started

TBD

## Risks / Issues

 - There is no throttling so someone can just flood you with requests and take the app down or inflate your AWS costs
 - Old data are not cleaned up.
 - [Rocket](rocket) is pre `1.0`, so API is still unstable and there are some missing features (like OpenAPI, async and SSL is not yet considered production ready). I would probably put `nginx` in from of this if serving on the internet.
 - No CORS header, so the API cannot be used by other sites (if intended to be used)
 - No performance monitoring
 - The logging story for rocket is not yet well developed, so it might be a bit hard to aggregate logs. If an nginx is put in front, we can at least have http logs.
 - Localization?
 - ?

TBD


[rocket]: http://rocket.rs