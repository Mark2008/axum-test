set -e # pause script when error

# pull from git repo
echo "[Pulling from git repository...]"
git pull || { echo "Pull failed"; exit 1; }

# build binary file
echo "[Building binary file...]"
cargo build --release || { echo "Build failed"; exit 1; }

# kill running process
echo "[Killing current server process...]"
(killall axum-test && sleep 1) || echo "No running process found"

# start serving server
echo "[Starting new server process...]"
nohup ./target/release/axum-test &
echo "[Server Started!]"