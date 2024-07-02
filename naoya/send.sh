#!/bin/sh
curl -X POST -H "Authorization: Bearer 6d48a347-10a8-41c5-ac07-d7652af6aabf" --data-binary @$1 "https://boundvariable.space/communicate" | python decode.py
