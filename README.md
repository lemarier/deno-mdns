# event-kit

Simple module for implementing event subscription APIs

[![Test CI](https://github.com/lemarier/deno-event-kit/workflows/Test%20CI/badge.svg)](https://github.com//lemarier/deno-event-kit/actions)

```ts
import { Emitter } from "https://deno.land/x/event_kit/mod.ts";

const emitter = new Emitter();

// Subscribe to this event
emitter.on("did-change-name", (name: string) => {
  console.log(name);
});

// Trigger the event
emitter.emit("did-change-name", "david");

// Remove the emitter instance
emitter.dispose();
```

## API

### Emitter(options?)

Returns a new emitter instance.

#### .emit(key, value)

Notify a subscriber.

#### .on(key, callback)

Subscribe to an event.

#### .dispose()

Delete the event emitter instance.
