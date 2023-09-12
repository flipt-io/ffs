import { FliptApiClient } from "@flipt-io/flipt";
import { DEFAULT_NAMESPACE } from "@flipt-io/flipt/constants";
import { v4 as uuidv4 } from "uuid";

const client = new FliptApiClient({
  environment: "http://localhost:8080",
  auth: {
    credentials: {
      username: "YOUR_USERNAME",
      password: "YOUR_PASSWORD",
    },
  },
});

let response = await client.evaluation.variant({
  namespaceKey: DEFAULT_NAMESPACE,
  flagKey: "abc123",
  entityId: uuidv4(),
  context: {},
});

console.log("Received variant response from Flipt!", response);

response = await client.evaluation.boolean({
  namespaceKey: DEFAULT_NAMESPACE,
  flagKey: "abc123",
  entityId: uuidv4(),
  context: {},
});

console.log("Received boolean response from Flipt!", response);
