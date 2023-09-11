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

const response = await client.evaluation.variant({
  namespaceKey: DEFAULT_NAMESPACE,
  flagKey: "abc123",
  entityId: uuidv4(),
  context: {},
});

console.log("Received response from Flipt!", response);
