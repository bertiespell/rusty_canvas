# Rusty Canvas

A drawing application and server, built in Rust.
It supports ASCII and Unicode.

## Building from source

You'll first need to install Rust - following instructions found [here](https://www.rust-lang.org/tools/install).

To run the project:
```bash
cargo run
```

The server is now running on [localhost:8080](`localhost:8080`).

To test:
```bash
cargo test
```

To build documentation:
```bash
cargo doc --open
```

This should open the documentation in your browser, or you can open:
`/rusty_canvas/target/doc/rusty_canvas/index.html`

## Painting

The application supports two draw operations via post requests made to either `/drawrectangle` and `/floodfill`.

Here is a helpful test palette: 🟥🟧🟨🟩🟦🟪🟫⬛⬜

Requests must adhere to the following structure

```json
// Post request body made to /drawrectangle
{
    "position": { "x": 6, "y": 6 },
    "dimensions":{ "width": 4, "height": 5 },
    "fill_character": "none", 
    "outline_character": "🟨"
}

// Post request body made to /floodfill
{
    "position": { "x": 2, "y": 2 },
    "fill_character": "🟦", 
}
```

The fill_character and outline_character fields accept any valid unicode character or the string "none".

You can use curl from the terminal to make the above requests:

```bash
curl --location --request POST 'localhost:8080/drawrectangle' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "position": { "x": 6, "y": 6 },
    "dimensions":{ "width":4, "height":5 },
    "fill_character": "none", 
    "outline_character": "🟨"
}'
```

```bash
curl --location --request POST 'localhost:8080/floodfill' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "position": { "x": 5, "y": 4 },
    "fill_character": "🟦"
}'
```

## Canvas Options

The canvas defaults to a size of 30 x 30, with "⬛" as the blank character.
You can change this by passing in width, height and blank_character arguments to cargo run.
If you'd like to run the canvas application with ASCII, you can pass " " as the blank character.

```bash
cargo run 20 30 🟪
```
The above creates a new canvas with width = 20, height = 30 and 🟪 as the blank character.

## Viewing the canvas

The canvas is statically hosted at [localhost:8080](`localhost:8080`) (page refresh currently required).

It is stored locally in `canvas_data.txt`, so you can also view this.

Or it is returned as the body to a succesful POST request.