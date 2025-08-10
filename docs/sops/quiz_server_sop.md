# SOP: Quiz Server

## 1. Purpose

To standardize the process of interacting with the quiz server, which is used to sample the model and update its weights.

## 2. Scope

This SOP applies to all interactions with the quiz server.

## 3. Procedure

### 3.1. Getting a Quiz Question

To get a quiz question, send a GET request to the `/quiz` endpoint:

```bash
curl http://127.0.0.1:3000/quiz
```

The server will respond with a JSON object containing the question's ID and text:

```json
{
  "id": 0,
  "text": "What is the capital of France?"
}
```

### 3.2. Submitting an Answer

To submit an answer, send a POST request to the `/answer` endpoint with a JSON payload containing the `question_id` and the `submitted_answer`:

```bash
curl -X POST -H "Content-Type: application/json" -d '{"question_id":0,"submitted_answer":"Paris"}' http://127.0.0.1:3000/answer
```

The server will respond with a JSON object indicating whether the answer was correct:

```json
{
  "correct": true
}
```

## 4. Expected Outcome

A robust and interactive quiz server that can be used to sample the model and update its weights based on user feedback.
