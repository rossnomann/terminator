# TERMINATOR

A terminator bot for telegram groups

When a new chat member joined a group, this bot restricts all permissions for that user.
User should answer a question before he can start chatting.
See configuration example below for more information.

## Installation

Download binary:

```
$ curl -L https://github.com/rossnomann/terminator/releases/download/0.1.3/terminator-0.1.3_x86_64-linux-gnu --output terminator
$ chmod +x terminator
```

## Usage

Create `config.yaml`:

```
token: 'YOUR-BOT-TOKEN-HERE'
# webhook_address: '127.0.0.1:8080'  # optional webhook address to run server on
# webhook_path: '/7260a3bfd7ba450b964fd486b9c9b84b'  # optional webhook path to get updates on; default - '/'
# if webhook address is not set, bot will receive updates via longpolling
chats:
  - chat_id: -1001234 # An integer ID of the target chat
    question: '{{user}}, are you a bot?'  # Question to ask; {{user}} is a user mention.
    buttons:
      - label: 'Yes'
        is_right: true  # permissions allowed
      - label: 'No'
        is_right: false  # permissions denied
    response_timeout: 10  # timeout in seconds; question will be deleted after
    # Optional parameters:
    # notification:
    #   right: 'Welcome!'  # notification when target user pressed right button
    #   wrong: 'Good luck'  # notification when target user pressed wrong button
    #   forbidden: 'Forbidden'  # notification when other user pressed any button
    # question_timeout: 1  # timeout in seconds; question will be send after this timeout; 0 - default
    # action:
    #   wrong: kick  # action when user respond with wrong answer; 'kick' or 'restrict'; default - restrict
    #   timeout: restrict  # action when user did not press any button; 'kick' or 'restrict'; default - restrict
```

Run:

```
$ ./terminator config.yaml
```

## Changelog

### 0.1.3 (19.04.2020)

- Keep old permissions when user leaved from a chat.

### 0.1.2 (26.03.2020)

- Fixed newline rendering.
- Reply to `new_chat_member` message when sending a question.

### 0.1.1 (21.03.2020)

- Added `action.wrong` and `action.timeout` options.

### 0.1.0 (16.03.2020)

- First release.

## LICENSE

The MIT License (MIT)
