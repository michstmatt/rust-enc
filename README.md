# Rust Encryption Libary

## Running

```sh
> # Help
> ./rust-aes
Expected either enc or dec followed by a password and value

> # To encrypt
> ./rust-aes enc password "text to encrypt"
B3pgeZ6uaI/D+sR7TSC/IA==

> # To Decrypt
> ./rust-aes dec password "B3pgeZ6uaI/D+sR7TSC/IA=="
text to encrypt

> ./rust-aes enc password "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
ardK/bgv9/3CEi5r8rS7bYwZYmjrWRmOTPpIXiqwxGe2P0RQrhGCTn0KNWA8iSOnNzxkDGW+BlK7JJhHcOc4Bw/kfCwfTfCbau/gmHegaTDmN+Yxb/fRtbWaRou9i/yeRBFVzJwE/clJfgfWizmIr8kNTZrw6ujPKrdc7TMZE0Vl284/bAq17wcLHpIvdNfkDPVoWzrucp1j4bSFmk34ViihDSTyaUE5OcIK8XEW0EcYsaf/RmdhgCk6xUYiHfs2I/xPVOZ8nag4jvnjr0KNW1g7r8dgYr4MXGDR+9ZqVCSYyx0lgho97kAK3TU1F3lqsCT2B2+CgxKVpvS/cwmDgse9EXuyWf9PY4cT5h+EDMRybEpur8u4A9tWSZgavVEu+bQXhbqYUIggAhDP2BIndLAY8+uqUjJXUJRtkq63yvroHPsThQqhsgLzNyfE13xJo+bAECYkwaBhYeiljbDBgzrD5IECaPBHhg8jYeyXZmAC8w7RIxEi0T33+p2VtiItDFedWqsRK+EgwVTPRfTVoIVF/BAyWcO7RNQloSvXB8MsXLrNLzC6NZ/5irVQ4EMXRl7qHXDxLA1pGLv6srjsJQ==

> ./rust-aes dec password "ardK/bgv9/3CEi5r8rS7bYwZYmjrWRmOTPpIXiqwxGe2P0RQrhGCTn0KNWA8iSOnNzxkDGW+BlK7JJhHcOc4Bw/kfCwfTfCbau/gmHegaTDmN+Yxb/fRtbWaRou9i/yeRBFVzJwE/clJfgfWizmIr8kNTZrw6ujPKrdc7TMZE0Vl284/bAq17wcLHpIvdNfkDPVoWzrucp1j4bSFmk34ViihDSTyaUE5OcIK8XEW0EcYsaf/RmdhgCk6xUYiHfs2I/xPVOZ8nag4jvnjr0KNW1g7r8dgYr4MXGDR+9ZqVCSYyx0lgho97kAK3TU1F3lqsCT2B2+CgxKVpvS/cwmDgse9EXuyWf9PY4cT5h+EDMRybEpur8u4A9tWSZgavVEu+bQXhbqYUIggAhDP2BIndLAY8+uqUjJXUJRtkq63yvroHPsThQqhsgLzNyfE13xJo+bAECYkwaBhYeiljbDBgzrD5IECaPBHhg8jYeyXZmAC8w7RIxEi0T33+p2VtiItDFedWqsRK+EgwVTPRfTVoIVF/BAyWcO7RNQloSvXB8MsXLrNLzC6NZ/5irVQ4EMXRl7qHXDxLA1pGLv6srjsJQ=="
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
```
