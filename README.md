# Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder.
If you chose to develop with the router feature, you will also have a `views` folder.

### Tailwind
1. Install tailwindcss cli: [npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm](https://tailwindcss.com/docs/installation/tailwind-cli)
2. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

