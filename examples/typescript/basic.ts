import { FliptApiClient } from "@flipt-io/flipt";
import { DEFAULT_NAMESPACE } from "@flipt-io/flipt/constants";

const client = new FliptApiClient({
  environment: "http://localhost:8080",
  auth: {
    credentials: {
      username: "YOUR_USERNAME",
      password: "YOUR_PASSWORD",
    },
  },
});

let response = await client.flags.get({
  namespaceKey: DEFAULT_NAMESPACE,
  key: "abc123",
});

console.log("Received response from Flipt!", response);
