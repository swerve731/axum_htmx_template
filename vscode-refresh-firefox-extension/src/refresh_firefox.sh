# i set a cusom shortcut to run this script when i save the css

WID=$(xdotool search --name "Mozilla Firefox" | head -1)
if [ -n "$WID" ]; then
  xdotool windowactivate "$WID"
  sleep 0.1
  xdotool key Ctrl+r
else
  echo "Error: Could not find Firefox window." >&2
fi