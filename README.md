# Epic games store scraper

This scraper is a small part of a larger project and is meant to scrap all the games from the epic games store.

One issue with the store is that it uses React to render the frontend, meaning we aren't able to make a requests to the different URL and process the DOM as no games will be available.

The work around is a bit meh but I am new to Rust and I am trying my best.

I created a small API with [hyper](https://hyper.rs/) that will process the DOM into a array of games using this format;

```ts
Array<{
  url: String,
  name: String,
  id: String,
  type: "product" | "bundles"
}>
```

## What is required?

- [Postman](https://www.postman.com/)
- [Rust](https://www.rust-lang.org/)

## How to scrape?

1. Clone this repo and run `cargo run`, that will start a local server at `http://127.0.0.1:3000/`.
2. Open Postman and create a **POST** request with a endpoint of `http://127.0.0.1:3000/scrape`.
3. Open [Epic Games](https://www.epicgames.com/store/en-US/browse?sortBy=title&sortDir=ASC&pageSize=1000), you are able to do this for individual categories or just all of them. Remember to set the `pageSize` to 1000 (as high as it can go).
4. Run the JS below and copy the output.
5. Back in Postman, paste the output in `Body > Raw` and make sure it is set to `JSON`.
6. Send the request.

<details>
  <summary>JS Code</summary>

```js
(function() {
  console.log(
    JSON.stringify({
      dom: document.getElementById("dieselReactWrapper").outerHTML
    })
  )
})()
```
</details>

That should create a file on your system called `scrapped-text.json` with the scrapped data.

At this point you are feel to do what you want with this data.

## What am I doing with it?

> Coming soon...
