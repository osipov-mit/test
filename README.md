## Task

1. Upload this program to the Vara Test Network using `https://idea.gear-tech.io`
2. Develop a backend service with the following functionality
   1. Send `Ping` message to the uploaded program on request and save the sent message with its id to the database
   2. Receive a response from the program and save it to the database
   3. On request, give statistics on the number of messages sent, `Pong` message received from the program and error messages received from the program.

---

`@gear-js/api` - [library](https://github.com/gear-tech/gear-js/tree/main/api) for working with blockchain
`@polkadot/api` - library necessary for `@gear-js/api` work.

[How to send a message to a program](https://github.com/gear-tech/gear-js/tree/main/api)
[How to calculate gas to send a message](https://github.com/gear-tech/gear-js/tree/main/api)
[How to listen to blockchain events](https://github.com/gear-tech/gear-js/tree/main/api)
_To receive response from the program you need to listen to `UserMessageSent` event_
[How to create a keyring to sign a transaction](https://github.com/gear-tech/gear-js/tree/main/api)
[How to get metadata of a program](https://github.com/gear-tech/gear-js/tree/main/api#getting-metadata)
_Metadata is necessary for encoding and decoding messages sent/received to/from the program_

---

`test_task.opt.wasm` - code of the program
`test_task.meta.txt` - metadata of the program

Types of the program messages:
`String` - type of the message to be sent to the program
`Result<String, u16>` - type of the message that program sends in response
