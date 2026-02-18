# Events Module (`std/events`)

The `events` module provides a native, high-performance event emitter implementation.
It allows objects to subscribe to and emit events, managing lists of callbacks efficiently in the runtime heap.

To use this module, you must activate it with the `#<events>` directive.

## Usage

```javascript
#<events>

// Create a new emitter
let emitter = events_new();

// Define a callback
function onData(data) {
    println("Received: " + data);
}

// Subscribe
events_on(emitter, "data", onData);

// Emit (returns list of callbacks to execute)
let callbacks = events_emit(emitter, "data");
for (cb in callbacks) {
    cb("Hello World");
}

// Unsubscribe
events_off(emitter, "data", onData);
```

## API Reference

### `events_new()`
Creates a new event emitter object.
- **Returns**: `Object` - A new emitter instance.

### `events_on(emitter, event, callback)`
Subscribes a callback function to an event.
- **emitter**: `Object` - The emitter instance created by `events_new()`.
- **event**: `String` - The name of the event.
- **callback**: `Function` - The function to be called when the event is emitted.

### `events_off(emitter, event, callback)`
Unsubscribes a callback function from an event.
- **emitter**: `Object` - The emitter instance.
- **event**: `String` - The name of the event.
- **callback**: `Function` - The function to remove.

### `events_emit(emitter, event)`
Retrieves the list of callbacks subscribed to an event.
- **emitter**: `Object` - The emitter instance.
- **event**: `String` - The name of the event.
- **Returns**: `Array<Function>` - A list of callback functions. You must iterate over this list and call them manually.

> [!NOTE]
> The `events_emit` function does **not** execute the callbacks directly. It returns them so the Dryad code can execute them. This preserves the execution context and allows for flexible argument passing.
