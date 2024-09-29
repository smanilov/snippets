# snippets
Web app to record, edit, and view chronological notes on various projects

`client/`: contains the FE of the app, written in JS + Tailwind + SolidJS + Vite

`server/`: contains the BE of the app, written in Rust

## Try it out

Once you checkout the repo, run the backend like so:

```
cd server && cargo run
```

Note: this requires that you have cargo installed. Recommended: use rustup.

Once the server is running, run the frontend like so (in a separate terminal):

```
cd client && npm install && npm run dev
```

Note: this requires that you have npm installed. Recommended: use nvm.

Once both server and client are running, open the web interface (localhost:3000)
and enter any string as access key. You will receive an "all good" message from
the BE.

## Complete script

Here's a complete script that works in a containerized Ubuntu:

```
# download and run an empty ubuntu container
docker run -it ubuntu:24.04 -p 11840:11840 -p 3000:3000 /bin/bash

# inside the container:
apt update && apt upgrade -y
apt install -y curl

# installing rustup
# this will require interactivity, stay sharp
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
rustup default stable
apt install -y build-essential

# installing nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion
nvm install node

# get the repo
apt install -y git
git clone https://github.com/smanilov/snippets.git

# run the backend
pushd snippets/server/
cargo build && target/debug/snippet-server &
popd
# run the frontend
cd snippets/client/
npm install && npm run dev
```

I couldn't figure out how to expose the 3000 port from the interactive
container, so you can also view the FE, not just run it. But the point is to
demonstrate the dependencies needed to build and run this project. You don't
really need docker for this.
