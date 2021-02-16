<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://raw.githubusercontent.com/EstebanBorai/okku-cli/main/docs/logo.png" height="120" width="120" />
  </div>
  <h1 align="center">okku-cli</h1>
  <h4 align="center">
    Terminal Front-End for Okku's chat server with text-only capabilities and single chat room per session
  </h4>
</div>

## Requirements

- [Okku Server](https://github.com/EstebanBorai/okku-server)
- [Rust](https://rustup.rs)


## Usage

> In order to consume this front-end, an instance of [Okku Server](https://github.com/EstebanBorai/okku-server) must be running in the background.

`okku-cli` provides a terminal UI to consume Okku Server chat solution. In order to consume the service
you will need:

- At least 2 users (including yours) for Okku Server
- Okku Server running in the background

Let's walk through how is established a chat session:

1. Run Okku's chat server instance as shown [here](https://github.com/EstebanBorai/okku-server#getting-started)

2. Create your user and store the token and the id of the returned entry somewhere to use in the future

```sh
curl \
  --header "Content-Type: application/json" \
  --request POST \
  --data '{"name":"estebanborai","password":"root", "email": "estebanborai@okku.com"}' \
  http://localhost:3000/api/v1/auth/signup
```

3. Repeat the step two but replace your user details with a new user details, this
will be the second participant of our chat. The ID returned from this request is
required to create our chat room in the next section, please make sure you keep both
IDs.

4. Now we are going to create a new chat room, specify both users ids on the following
request to Okku's Server, we will also need the ID returned by this request on step 5,
please make sure you keep it as well:

```sh
curl \
  --header "Content-Type: application/json" \
  --request POST \
  --data '{"participants_ids": ["<USER ID>", "<SECOND USER ID>"]}' \
  http://localhost:3000/api/v1/chats
```

> You are allowed to specify as many participants as you like

5. With the chat room ID in place with our participants in it, we are ready to start
a chat session! To achieve this, we need to run `okku-cli` as follows:

```sh
OKKU_HOST=http://localhost:3000 OKKU_CHAT=<OUR CHAT ID> cargo run -- <USERNAME> <PASSWORD>
```

If you are working on some feature/fix is recomended to run the command piping the `stderr` output
to a file as follows:

```sh
OKKU_HOST=http://localhost:3000 OKKU_CHAT=<OUR CHAT ID> cargo run -- <USERNAME> <PASSWORD> 2> error.log
```

6. You are good to go! As you may think, theres many features we could implement on this solution as well,
some of them are:

- Capability to fetch user chats instead of specifying `OKKU_CHAT` environment variable
- Capability to create chat rooms from the CLI itself
- Capability to switch beetween chats in the same session
- Capability to authenticate on runtime

I'm sure theres many more features we could implement here! The purposes of this project are
educational, so feel free to open a pull request or an issue, I'm glad to help and share!

## License

This project is licensed under the MIT License
