1. User communicate -> Printer
2. User input -> Reader
3. User configuration ->

## Messaging API Examples

### Send Message

```
curl -v -X POST https://api.line.me/v2/bot/message/push \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer zSzXAm8PobDE92hC1edXBpPcVwbdYTk7rOvrHtvqzUb/VibCG491TfFHi5TnNvmA4D4AZLZ2znIRXXdILuLzdaKy3oohB1wUdHf79Xq1cyJQBCqJK8L5kwXetqEuO43hd5jLh252wE5AzIb/7oDb+wdB04t89/1O/w1cDnyilFU=' \
-d '{
    "to": "U4af4980629...",
    "messages":[
        {
            "type":"text",
            "text":"Hello, world1"
        },
        {
            "type":"text",
            "text":"Hello, world2"
        }
    ]
}'
```

### Get friends

```
curl -v -X GET https://api.line.me/v2/bot/followers/ids?start={continuationToken} \
-H 'Authorization: Bearer zSzXAm8PobDE92hC1edXBpPcVwbdYTk7rOvrHtvqzUb/VibCG491TfFHi5TnNvmA4D4AZLZ2znIRXXdILuLzdaKy3oohB1wUdHf79Xq1cyJQBCqJK8L5kwXetqEuO43hd5jLh252wE5AzIb/7oDb+wdB04t89/1O/w1cDnyilFU='
```