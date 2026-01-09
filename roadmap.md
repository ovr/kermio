# Kermio Hermes API Roadmap

This document provides a comprehensive overview of the public APIs available in the Hermes JavaScript engine (from hermes-vendor), and tracks which APIs have been exposed in the Kermio Rust bindings.

## Table of Contents

- [JSI (JavaScript Interface) API](#jsi-javascript-interface-api)
- [Hermes-Specific APIs](#hermes-specific-apis)
- [Currently Exposed in Kermio](#currently-exposed-in-kermio)
- [Implementation Roadmap](#implementation-roadmap)

---

## JSI (JavaScript Interface) API

JSI (JavaScript Interface) is Facebook's C++ abstraction layer for JavaScript engines. It's located in `API/jsi/jsi/jsi.h`.

### Runtime Class

The main entry point for interacting with a JavaScript runtime.

#### Code Evaluation

```cpp
// Evaluate JavaScript code from a buffer
virtual Value evaluateJavaScript(
    const std::shared_ptr<const Buffer>& buffer,
    const std::string& sourceURL) = 0;

// Prepare JavaScript for optimized execution
virtual std::shared_ptr<const PreparedJavaScript> prepareJavaScript(
    const std::shared_ptr<const Buffer>& buffer,
    std::string sourceURL) = 0;

// Execute prepared JavaScript
virtual Value evaluatePreparedJavaScript(
    const std::shared_ptr<const PreparedJavaScript>& js) = 0;
```

**Status:** ✅ Partially exposed via `Runtime::eval()` and `Runtime::eval_with_result()`

#### Global Object and Description

```cpp
// Get the global object
virtual Object global() = 0;

// Get runtime description
virtual std::string description() = 0;

// Check if runtime is debuggable
virtual bool isInspectable() = 0;
```

**Status:** ❌ Not exposed

#### Microtask Queue

```cpp
// Queue a microtask
virtual void queueMicrotask(const jsi::Function& callback) = 0;

// Drain microtasks
virtual bool drainMicrotasks(int maxMicrotasksHint = -1) = 0;
```

**Status:** ❌ Not exposed

#### Instrumentation

```cpp
// Get instrumentation interface for metrics
virtual Instrumentation& instrumentation();
```

**Status:** ❌ Not exposed

#### Runtime Data Storage

```cpp
// Store custom data in the runtime
void setRuntimeData(const UUID& uuid, const std::shared_ptr<void>& data);
std::shared_ptr<void> getRuntimeData(const UUID& uuid);
```

**Status:** ❌ Not exposed

### Value Types

#### PropNameID

Property name identifier for efficient property access.

```cpp
class PropNameID {
  // Create from ASCII
  static PropNameID forAscii(Runtime& runtime, const char* str, size_t length);
  static PropNameID forAscii(Runtime& runtime, const char* str);
  static PropNameID forAscii(Runtime& runtime, const std::string& str);

  // Create from UTF-8
  static PropNameID forUtf8(Runtime& runtime, const uint8_t* utf8, size_t length);
  static PropNameID forUtf8(Runtime& runtime, const std::string& utf8);

  // Create from UTF-16
  static PropNameID forUtf16(Runtime& runtime, const char16_t* utf16, size_t length);
  static PropNameID forUtf16(Runtime& runtime, const std::u16string& str);

  // Create from String or Symbol
  static PropNameID forString(Runtime& runtime, const jsi::String& str);
  static PropNameID forSymbol(Runtime& runtime, const jsi::Symbol& sym);

  // Convert to string
  std::string utf8(Runtime& runtime) const;
  std::u16string utf16(Runtime& runtime) const;

  // Compare
  static bool compare(Runtime& runtime, const PropNameID& a, const PropNameID& b);
};
```

**Status:** ✅ Exposed in jsi-rs as `JSPropNameID::new()` (UTF-8 only)

#### Symbol

ES6 Symbol type.

```cpp
class Symbol {
  // Compare symbols
  static bool strictEquals(Runtime& runtime, const Symbol& a, const Symbol& b);

  // Convert to string (like Symbol.toString())
  std::string toString(Runtime& runtime) const;
};
```

**Status:** ❌ Not exposed

#### BigInt

Arbitrary-precision integers.

```cpp
class BigInt {
  // Create from integers
  static BigInt fromInt64(Runtime& runtime, int64_t value);
  static BigInt fromUint64(Runtime& runtime, uint64_t value);

  // Compare
  static bool strictEquals(Runtime& runtime, const BigInt& a, const BigInt& b);

  // Convert to integers
  int64_t getInt64(Runtime& runtime) const;        // Truncates
  uint64_t getUint64(Runtime& runtime) const;       // Truncates
  bool isInt64(Runtime& runtime) const;
  bool isUint64(Runtime& runtime) const;
  int64_t asInt64(Runtime& runtime) const;          // Throws if lossy
  uint64_t asUint64(Runtime& runtime) const;        // Throws if lossy

  // Convert to string
  String toString(Runtime& runtime, int radix = 10) const;
};
```

**Status:** ✅ Exposed in jsi-rs as `JSBigInt::from_i64()` and `JSBigInt::from_u64()`

#### String

JavaScript string type.

```cpp
class String {
  // Create from various encodings
  static String createFromAscii(Runtime& runtime, const char* str, size_t length);
  static String createFromAscii(Runtime& runtime, const char* str);
  static String createFromAscii(Runtime& runtime, const std::string& str);
  static String createFromUtf8(Runtime& runtime, const uint8_t* utf8, size_t length);
  static String createFromUtf8(Runtime& runtime, const std::string& utf8);
  static String createFromUtf16(Runtime& runtime, const char16_t* utf16, size_t length);
  static String createFromUtf16(Runtime& runtime, const std::u16string& utf16);

  // Compare
  static bool strictEquals(Runtime& runtime, const String& a, const String& b);

  // Convert to C++ strings
  std::string utf8(Runtime& runtime) const;
  std::u16string utf16(Runtime& runtime) const;

  // Get string data efficiently (callback-based)
  template <typename CB>
  void getStringData(Runtime& runtime, CB& cb) const;
};
```

**Status:** ✅ Exposed in jsi-rs as `JSString::new()` (UTF-8 only)

#### Object

JavaScript object type.

```cpp
class Object {
  // Create objects
  explicit Object(Runtime& runtime);  // Create empty object
  static Object createFromHostObject(Runtime& runtime, std::shared_ptr<HostObject> ho);
  static Object create(Runtime& runtime, const Value& prototype);

  // Compare
  static bool strictEquals(Runtime& runtime, const Object& a, const Object& b);

  // Prototype manipulation
  void setPrototype(Runtime& runtime, const Value& prototype) const;
  Value getPrototype(Runtime& runtime) const;

  // Property access
  Value getProperty(Runtime& runtime, const char* name) const;
  Value getProperty(Runtime& runtime, const String& name) const;
  Value getProperty(Runtime& runtime, const PropNameID& name) const;
  Value getProperty(Runtime& runtime, const Value& name) const;

  bool hasProperty(Runtime& runtime, const char* name) const;
  bool hasProperty(Runtime& runtime, const String& name) const;
  bool hasProperty(Runtime& runtime, const PropNameID& name) const;
  bool hasProperty(Runtime& runtime, const Value& name) const;

  template <typename T>
  void setProperty(Runtime& runtime, const char* name, T&& value) const;
  template <typename T>
  void setProperty(Runtime& runtime, const String& name, T&& value) const;
  template <typename T>
  void setProperty(Runtime& runtime, const PropNameID& name, T&& value) const;
  template <typename T>
  void setProperty(Runtime& runtime, const Value& name, T&& value) const;

  void deleteProperty(Runtime& runtime, const char* name) const;
  void deleteProperty(Runtime& runtime, const String& name) const;
  void deleteProperty(Runtime& runtime, const PropNameID& name) const;
  void deleteProperty(Runtime& runtime, const Value& name) const;

  // Type checking
  bool isArray(Runtime& runtime) const;
  bool isArrayBuffer(Runtime& runtime) const;
  bool isFunction(Runtime& runtime) const;
  template <typename T = HostObject>
  bool isHostObject(Runtime& runtime) const;

  // Type conversions
  Array getArray(Runtime& runtime) const&;
  Array asArray(Runtime& runtime) const&;  // Throws if not array
  ArrayBuffer getArrayBuffer(Runtime& runtime) const&;
  Function getFunction(Runtime& runtime) const&;
  Function asFunction(Runtime& runtime) const&;  // Throws if not function

  // HostObject access
  template <typename T = HostObject>
  std::shared_ptr<T> getHostObject(Runtime& runtime) const;
  template <typename T = HostObject>
  std::shared_ptr<T> asHostObject(Runtime& runtime) const;  // Throws

  // Native state
  template <typename T = NativeState>
  bool hasNativeState(Runtime& runtime) const;
  template <typename T = NativeState>
  std::shared_ptr<T> getNativeState(Runtime& runtime) const;
  void setNativeState(Runtime& runtime, std::shared_ptr<NativeState> state) const;

  // Convenience methods
  Object getPropertyAsObject(Runtime& runtime, const char* name) const;
  Function getPropertyAsFunction(Runtime& runtime, const char* name) const;
  Array getPropertyNames(Runtime& runtime) const;

  // Memory pressure hint
  void setExternalMemoryPressure(Runtime& runtime, size_t amt) const;

  // instanceof check
  bool instanceOf(Runtime& rt, const Function& ctor) const;
};
```

**Status:** ✅ Exposed in jsi-rs as `JSObject::new()` (basic creation only)

#### WeakObject

Weak references to JavaScript objects.

```cpp
class WeakObject {
  // Create weak reference
  WeakObject(Runtime& runtime, const Object& o);

  // Get the object if still alive, otherwise undefined
  Value lock(Runtime& runtime) const;
};
```

**Status:** ❌ Not exposed

#### Array

JavaScript array type (extends Object).

```cpp
class Array : public Object {
  // Create array
  Array(Runtime& runtime, size_t length);

  // Get size
  size_t size(Runtime& runtime) const;
  size_t length(Runtime& runtime) const;

  // Element access
  Value getValueAtIndex(Runtime& runtime, size_t i) const;
  template <typename T>
  void setValueAtIndex(Runtime& runtime, size_t i, T&& value) const;

  // Create with elements
  template <typename... Args>
  static Array createWithElements(Runtime&, Args&&... args);
  static Array createWithElements(Runtime& runtime, std::initializer_list<Value> elements);
};
```

**Status:** ✅ Exposed in jsi-rs as `JSArray::new()` (basic creation only)

#### ArrayBuffer

Binary data buffers.

```cpp
class ArrayBuffer : public Object {
  // Create ArrayBuffer
  ArrayBuffer(Runtime& runtime, std::shared_ptr<MutableBuffer> buffer);

  // Get size and data pointer
  size_t size(Runtime& runtime) const;
  size_t length(Runtime& runtime) const;
  uint8_t* data(Runtime& runtime) const;
};
```

**Status:** ❌ Not exposed

#### Function

Callable JavaScript functions (extends Object).

```cpp
class Function : public Object {
  // Create from host function
  static Function createFromHostFunction(
      Runtime& runtime,
      const jsi::PropNameID& name,
      unsigned int paramCount,
      jsi::HostFunctionType func);

  // Call function
  Value call(Runtime& runtime, const Value* args, size_t count) const;
  Value call(Runtime& runtime, std::initializer_list<Value> args) const;
  template <typename... Args>
  Value call(Runtime& runtime, Args&&... args) const;

  // Call with explicit 'this'
  Value callWithThis(Runtime& Runtime, const Object& jsThis, const Value* args, size_t count) const;
  Value callWithThis(Runtime& runtime, const Object& jsThis, std::initializer_list<Value> args) const;
  template <typename... Args>
  Value callWithThis(Runtime& runtime, const Object& jsThis, Args&&... args) const;

  // Call as constructor
  Value callAsConstructor(Runtime& runtime, const Value* args, size_t count) const;
  Value callAsConstructor(Runtime& runtime, std::initializer_list<Value> args) const;
  template <typename... Args>
  Value callAsConstructor(Runtime& runtime, Args&&... args) const;

  // Check if host function
  bool isHostFunction(Runtime& runtime) const;
  HostFunctionType& getHostFunction(Runtime& runtime) const;
};
```

**Status:** ❌ Not exposed

#### Value

Universal JavaScript value type.

```cpp
class Value {
  // Constructors
  Value();                              // undefined
  Value(std::nullptr_t);                // null
  Value(bool b);                        // boolean
  Value(double d);                      // number
  Value(int i);                         // number
  Value(Symbol&& sym);                  // symbol
  Value(BigInt&& bigint);               // bigint
  Value(String&& str);                  // string
  Value(Object&& obj);                  // object
  Value(Runtime& runtime, const Value& value);  // Copy

  // Static constructors
  static Value undefined();
  static Value null();
  static Value createFromJsonUtf8(Runtime& runtime, const uint8_t* json, size_t length);

  // Comparison
  static bool strictEquals(Runtime& runtime, const Value& a, const Value& b);

  // Type checking
  bool isUndefined() const;
  bool isNull() const;
  bool isBool() const;
  bool isNumber() const;
  bool isString() const;
  bool isBigInt() const;
  bool isSymbol() const;
  bool isObject() const;

  // Extract values (assert if wrong type)
  bool getBool() const;
  double getNumber() const;
  Symbol getSymbol(Runtime& runtime) const&;
  BigInt getBigInt(Runtime& runtime) const&;
  String getString(Runtime& runtime) const&;
  Object getObject(Runtime& runtime) const&;

  // Extract values (throw if wrong type)
  bool asBool() const;
  double asNumber() const;
  Symbol asSymbol(Runtime& runtime) const&;
  BigInt asBigInt(Runtime& runtime) const&;
  String asString(Runtime& runtime) const&;
  Object asObject(Runtime& runtime) const&;

  // Convert to string
  String toString(Runtime& runtime) const;
};
```

**Status:** ✅ Exposed in jsi-rs as `JSValue` with type checking methods

### Host Interop

#### HostFunctionType

Function signature for C++ functions callable from JavaScript.

```cpp
using HostFunctionType = std::function<
    Value(Runtime& rt, const Value& thisVal, const Value* args, size_t count)>;
```

**Status:** ❌ Not exposed

#### HostObject

Interface for exposing C++ objects to JavaScript.

```cpp
class HostObject {
  virtual ~HostObject();

  // Get property
  virtual Value get(Runtime&, const PropNameID& name);

  // Set property
  virtual void set(Runtime&, const PropNameID& name, const Value& value);

  // Get property names
  virtual std::vector<PropNameID> getPropertyNames(Runtime& rt);
};
```

**Status:** ❌ Not exposed

#### NativeState

Attach native state to JavaScript objects.

```cpp
class NativeState {
  virtual ~NativeState();
};
```

**Status:** ❌ Not exposed

### Buffers

#### Buffer

Immutable buffer for code or data.

```cpp
class Buffer {
  virtual ~Buffer();
  virtual size_t size() const = 0;
  virtual const uint8_t* data() const = 0;
};

class StringBuffer : public Buffer {
  StringBuffer(std::string s);
};
```

**Status:** ❌ Not exposed

#### MutableBuffer

Mutable buffer for ArrayBuffer.

```cpp
class MutableBuffer {
  virtual ~MutableBuffer();
  virtual size_t size() const = 0;
  virtual uint8_t* data() = 0;
};
```

**Status:** ❌ Not exposed

#### PreparedJavaScript

Pre-compiled/optimized JavaScript code.

```cpp
class PreparedJavaScript {
  virtual ~PreparedJavaScript() = 0;
};
```

**Status:** ❌ Not exposed

### Utilities

#### Scope

RAII scope for eager resource cleanup.

```cpp
class Scope {
  explicit Scope(Runtime& rt);
  ~Scope();

  template <typename F>
  static auto callInNewScope(Runtime& rt, F f) -> decltype(f());
};
```

**Status:** ❌ Not exposed

### Exceptions

```cpp
// Base exception
class JSIException : public std::exception;

// Native API errors
class JSINativeException : public JSIException;

// JavaScript errors
class JSError : public JSIException {
  JSError(Runtime& r, Value&& value);
  JSError(Runtime& rt, std::string message);
  JSError(Runtime& rt, std::string message, std::string stack);

  const std::string& getStack() const;
  const std::string& getMessage() const;
  const jsi::Value& value() const;
};
```

**Status:** ❌ Not exposed (C++ exceptions converted to `Result<T, String>`)

### Serialization (JSI_UNSTABLE)

```cpp
class Serialized {
  virtual void* getPrivate(const void* secretAddr) = 0;
  virtual ~Serialized();
};

class ISerialization : public ICast {
  // Serialize/deserialize values
  virtual std::shared_ptr<Serialized> serialize(Value& value) = 0;
  virtual Value deserialize(const std::shared_ptr<Serialized>& serialized) = 0;

  // Serialize with transferable objects
  virtual std::unique_ptr<Serialized> serializeWithTransfer(
      Value& value,
      const Array& transferList) = 0;
  virtual Array deserializeWithTransfer(
      std::unique_ptr<Serialized>& serialized) = 0;
};
```

**Status:** ❌ Not exposed

---

## Hermes-Specific APIs

Hermes provides additional APIs beyond JSI for engine-specific functionality.

### IHermesRootAPI

Global Hermes API for static methods (from `API/hermes/hermes.h`).

```cpp
class IHermesRootAPI : public jsi::ICast {
  // Create Hermes runtime
  virtual std::unique_ptr<HermesRuntime> makeHermesRuntime(
      const ::hermes::vm::RuntimeConfig &runtimeConfig) = 0;

  // Bytecode validation
  virtual bool isHermesBytecode(const uint8_t *data, size_t len) = 0;
  virtual uint32_t getBytecodeVersion() = 0;
  virtual bool hermesBytecodeSanityCheck(
      const uint8_t *data,
      size_t len,
      std::string *errorMessage = nullptr) = 0;

  // Bytecode optimization
  virtual void prefetchHermesBytecode(const uint8_t *data, size_t len) = 0;
  virtual std::pair<const uint8_t *, size_t> getBytecodeEpilogue(
      const uint8_t *data,
      size_t len) = 0;

  // Global fatal handler
  virtual void setFatalHandler(void (*handler)(const std::string &)) = 0;

  // Sampling profiler (global)
  virtual void enableSamplingProfiler(double meanHzFreq = 100) = 0;
  virtual void disableSamplingProfiler() = 0;
  virtual void dumpSampledTraceToFile(const std::string &fileName) = 0;
  virtual void dumpSampledTraceToStream(std::ostream &stream) = 0;

  // Code coverage
  virtual std::unordered_map<std::string, std::vector<std::string>>
  getExecutedFunctions() = 0;
  virtual bool isCodeCoverageProfilerEnabled() = 0;
  virtual void enableCodeCoverageProfiler() = 0;
  virtual void disableCodeCoverageProfiler() = 0;
};
```

**Status:** ✅ `isHermesBytecode` exposed as `Runtime::is_hermes_bytecode()`

### IHermes

Runtime-specific Hermes methods (from `API/jsi/jsi/hermes-interfaces.h`).

```cpp
class IHermes : public jsi::ICast {
  // Debug evaluation
  struct DebugFlags {};
  virtual void debugJavaScript(
      const std::string& src,
      const std::string& sourceURL,
      const DebugFlags& debugFlags) = 0;

  // Root API access
  virtual ICast* getHermesRootAPI() = 0;

  // Profiling
  virtual sampling_profiler::Profile dumpSampledTraceToProfile() = 0;
  virtual void sampledTraceToStreamInDevToolsFormat(std::ostream& stream) = 0;
  virtual void registerForProfiling() = 0;
  virtual void unregisterForProfiling() = 0;

  // Timezone
  virtual void resetTimezoneCache() = 0;

  // Segment loading
  virtual void loadSegment(
      std::unique_ptr<const jsi::Buffer> buffer,
      const jsi::Value& context) = 0;

  // Unique IDs
  virtual uint64_t getUniqueID(const jsi::Object& o) const = 0;
  virtual uint64_t getUniqueID(const jsi::BigInt& s) const = 0;
  virtual uint64_t getUniqueID(const jsi::String& s) const = 0;
  virtual uint64_t getUniqueID(const jsi::PropNameID& pni) const = 0;
  virtual uint64_t getUniqueID(const jsi::Symbol& sym) const = 0;
  virtual uint64_t getUniqueID(const jsi::Value& val) const = 0;
  virtual jsi::Value getObjectForID(uint64_t id) = 0;

  // Execution tracing
  virtual const ::hermes::vm::GCExecTrace& getGCExecTrace() const = 0;
  virtual std::string getIOTrackingInfoJSON() = 0;

  // Debugger
  virtual debugger::Debugger& getDebugger() = 0;

  // Timeout/interruption
  virtual void asyncTriggerTimeout() = 0;
  virtual void watchTimeLimit(uint32_t timeoutInMs) = 0;
  virtual void unwatchTimeLimit() = 0;

  // Source maps
  virtual jsi::Value evaluateJavaScriptWithSourceMap(
      const std::shared_ptr<const jsi::Buffer>& buffer,
      const std::shared_ptr<const jsi::Buffer>& sourceMapBuf,
      const std::string& sourceURL) = 0;

  // SHUnit (Static Hermes)
  virtual jsi::Value evaluateSHUnit(SHUnitCreator shUnitCreator) = 0;
  virtual SHRuntime* getSHRuntime() noexcept = 0;

  // Unsafe VM access
  virtual void* getVMRuntimeUnsafe() const = 0;
};
```

**Status:** ❌ Not exposed

### RuntimeConfig

Configuration for Hermes runtime (from `public/hermes/Public/RuntimeConfig.h`).

```cpp
struct RuntimeConfig {
  // GC configuration
  GCConfig GCConfig;

  // Register stack
  PinnedHermesValue* RegisterStack = nullptr;
  unsigned MaxNumRegisters = 128 * 1024;
  unsigned NativeStackGap = 64 * 1024;

  // JIT and eval
  bool EnableJIT = false;
  bool EnableEval = true;
  bool VerifyEvalIR = false;
  bool OptimizedEval = false;
  bool AsyncBreakCheckInEval = true;

  // Language features
  bool ES6Proxy = true;
  bool ES6BlockScoping = false;
  bool EnableAsyncGenerators = false;
  bool Intl = true;
  bool MicrotaskQueue = false;
  bool EnableGenerator = true;

  // Profiling and debugging
  SynthTraceMode SynthTraceMode = SynthTraceMode::None;
  bool EnableSampledStats = false;
  bool EnableSampleProfiling = false;

  // Memory and optimization
  bool RandomizeMemoryLayout = false;
  unsigned BytecodeWarmupPercent = 0;
  bool TrackIO = false;

  // Internal features
  bool EnableHermesInternal = true;
  bool EnableHermesInternalTestMethods = false;

  // Compilation
  CompilationMode CompilationMode = CompilationMode::SmartCompilation;

  // Crash management
  std::shared_ptr<CrashManager> CrashMgr = new NopCrashManager;

  // Experiment flags
  uint32_t VMExperimentFlags = 0;
};

enum CompilationMode {
  SmartCompilation,
  ForceEagerCompilation,
  ForceLazyCompilation
};
```

**Status:** ✅ Fully exposed as `RuntimeConfig` and `RuntimeConfigBuilder`

### CompileJS

Standalone bytecode compilation (from `API/hermes/CompileJS.h`).

```cpp
// Diagnostic handler for compilation errors
class DiagnosticHandler {
  enum Kind { Error, Warning, Note };

  struct Diagnostic {
    Kind kind;
    int line;     // 1-based
    int column;   // 1-based
    std::string message;
    std::vector<std::pair<unsigned, unsigned>> ranges;
  };

  virtual void handle(const Diagnostic &diagnostic) = 0;
};

struct CompileJSOptions {
  bool optimize = true;
  unsigned inlineMaxSize = 50;
  bool emitAsyncBreakCheck = false;
  bool debug = false;
};

// Compile JavaScript to bytecode
bool compileJS(
    const std::string &str,
    const std::string &sourceURL,
    std::string &bytecode,
    bool optimize,
    bool emitAsyncBreakCheck,
    DiagnosticHandler *diagHandler,
    std::optional<std::string_view> sourceMapBuf = std::nullopt,
    bool debug = false);

bool compileJS(
    const std::string &str,
    std::string &bytecode,
    bool optimize = true);

bool compileJS(
    const std::string &str,
    const std::string &sourceURL,
    std::string &bytecode,
    const CompileJSOptions &options,
    DiagnosticHandler *diagHandler,
    std::optional<std::string_view> sourceMapBuf = std::nullopt);
```

**Status:** ✅ Exposed as `Runtime::compile_to_bytecode()` (basic version)

### GCConfig

Garbage collector configuration (from `public/hermes/Public/GCConfig.h`).

```cpp
struct GCConfig {
  // Heap size limits
  gcheapsize_t MinHeapSize;
  gcheapsize_t InitHeapSize;
  gcheapsize_t MaxHeapSize;

  // GC thresholds
  double OccupancyTarget;
  double EffectiveOccupancyTarget;

  // Allocation limits
  gcheapsize_t AllocationLimit;

  // Tripwire (memory threshold callback)
  gcheapsize_t TripwireLimit;
  std::function<void(GCTripwireContext&)> TripwireCallback;

  // GC behavior
  bool ShouldReleaseUnused;
  bool ShouldRecordStats;

  // Sanitization
  bool ShouldRandomizeAllocSpace;
};
```

**Status:** ✅ Partially exposed via `RuntimeConfigBuilder::heap_size()`

### Helper Functions

```cpp
// Create Hermes runtime
std::unique_ptr<HermesRuntime> makeHermesRuntime(
    const ::hermes::vm::RuntimeConfig &runtimeConfig);

std::unique_ptr<HermesRuntime> makeHermesRuntimeNoThrow(
    const ::hermes::vm::RuntimeConfig &runtimeConfig) noexcept;

// Thread-safe runtime wrapper
std::unique_ptr<jsi::ThreadSafeRuntime> makeThreadSafeHermesRuntime(
    const ::hermes::vm::RuntimeConfig &runtimeConfig);

// Hardened config for untrusted code
::hermes::vm::RuntimeConfig hardenedHermesRuntimeConfig();

// Root API accessor
jsi::ICast* makeHermesRootAPI();
```

**Status:** ✅ `makeHermesRuntime` used internally by `Runtime::new()`

---

## Currently Exposed in Kermio

### hermes-engine Crate

High-level Rust bindings:

✅ **Runtime**
- `Runtime::new(config)` - Create runtime
- `Runtime::eval(source, url)` - Evaluate JavaScript
- `Runtime::eval_with_result(source, url)` - Evaluate with result
- `Runtime::eval_bytecode(bytecode)` - Execute bytecode
- `Runtime::is_hermes_bytecode(data)` - Validate bytecode
- `Runtime::compile_to_bytecode(source, url)` - Compile to bytecode
- `Runtime::jsi_runtime()` - Get raw JSI runtime pointer
- `Runtime::jsi()` (unsafe feature) - Get JSRuntime wrapper

✅ **RuntimeConfig / RuntimeConfigBuilder**
- Full configuration support for all Hermes runtime options
- Heap size configuration
- Language feature toggles (eval, JIT, ES6 proxy, generators, etc.)
- Profiling and debugging options

### jsi-rs Crate

Low-level JSI bindings:

✅ **JSValue**
- Type checking (`is_undefined`, `is_null`, `is_bool`, `is_number`, `is_string`, `is_object`)
- Factory methods (`undefined()`, `null()`, `bool()`, `number()`)

✅ **JSRuntime**
- Wrapper around `facebook::jsi::Runtime`
- Safe pointer management

✅ **JSObject**
- `JSObject::new(runtime)` - Create empty object

✅ **JSArray**
- `JSArray::new(runtime, length)` - Create array with length

✅ **JSString**
- `JSString::new(runtime, data)` - Create string from UTF-8

✅ **JSBigInt**
- `JSBigInt::from_i64(runtime, value)` - Create from i64
- `JSBigInt::from_u64(runtime, value)` - Create from u64

✅ **JSPropNameID**
- `JSPropNameID::new(runtime, name)` - Create from UTF-8

---

## Implementation Roadmap

### Phase 1: Core Value Operations (High Priority)

Essential for basic JavaScript interop.

#### JSValue Conversions

```rust
impl JSValue {
    // Extract native Rust values
    pub fn as_bool(&self) -> Option<bool>;
    pub fn as_number(&self) -> Option<f64>;
    pub fn as_string(&self, runtime: &mut JSRuntime) -> Option<String>;
    pub fn as_bigint_i64(&self, runtime: &mut JSRuntime) -> Option<i64>;
    pub fn as_bigint_u64(&self, runtime: &mut JSRuntime) -> Option<u64>;

    // Convert to Object/Array/Function
    pub fn as_object(&self) -> Option<JSObject>;
    pub fn as_array(&self) -> Option<JSArray>;
    pub fn as_function(&self) -> Option<JSFunction>;
}
```

**Why:** Without value extraction, we can't get data from JavaScript to Rust.

#### Object Property Access

```rust
impl JSObject {
    // Get/set properties
    pub fn get_property(&self, runtime: &mut JSRuntime, name: &str)
        -> Result<JSValue, String>;
    pub fn set_property(&mut self, runtime: &mut JSRuntime, name: &str, value: JSValue)
        -> Result<(), String>;
    pub fn has_property(&self, runtime: &mut JSRuntime, name: &str) -> bool;
    pub fn delete_property(&mut self, runtime: &mut JSRuntime, name: &str)
        -> Result<(), String>;

    // Get property names
    pub fn get_property_names(&self, runtime: &mut JSRuntime) -> Result<JSArray, String>;
}
```

**Why:** Object property access is fundamental for JavaScript interop.

#### Array Operations

```rust
impl JSArray {
    // Element access
    pub fn get_value_at_index(&self, runtime: &mut JSRuntime, index: usize)
        -> Result<JSValue, String>;
    pub fn set_value_at_index(&mut self, runtime: &mut JSRuntime, index: usize, value: JSValue)
        -> Result<(), String>;
    pub fn length(&self, runtime: &mut JSRuntime) -> usize;

    // Create from elements
    pub fn create_with_elements(runtime: &mut JSRuntime, elements: &[JSValue])
        -> Result<JSArray, String>;
}
```

**Why:** Arrays are ubiquitous in JavaScript code.

#### String Utilities

```rust
impl JSString {
    // Convert to Rust String
    pub fn to_utf8(&self, runtime: &mut JSRuntime) -> String;
    pub fn to_utf16(&self, runtime: &mut JSRuntime) -> Vec<u16>;
    pub fn length(&self, runtime: &mut JSRuntime) -> usize;
}
```

**Why:** Need to convert JavaScript strings to Rust strings.

### Phase 2: Function Calls and Global Object (High Priority)

Enable calling JavaScript functions and accessing globals.

#### Function Support

```rust
pub struct JSFunction {
    inner: cxx::UniquePtr<ffi::JSIFunction>,
}

impl JSFunction {
    // Call function
    pub fn call(&self, runtime: &mut JSRuntime, args: &[JSValue])
        -> Result<JSValue, String>;
    pub fn call_with_this(&self, runtime: &mut JSRuntime, this: &JSObject, args: &[JSValue])
        -> Result<JSValue, String>;
    pub fn call_as_constructor(&self, runtime: &mut JSRuntime, args: &[JSValue])
        -> Result<JSValue, String>;

    // Create from host function
    pub fn create_from_host_function(
        runtime: &mut JSRuntime,
        name: &str,
        param_count: usize,
        func: Box<dyn Fn(&mut JSRuntime, &JSValue, &[JSValue]) -> Result<JSValue, String>>
    ) -> Result<JSFunction, String>;
}
```

**Why:** Calling JavaScript functions from Rust is essential.

#### Global Object Access

```rust
impl Runtime {
    // Get global object
    pub fn global(&mut self) -> JSObject;
}
```

**Why:** Access to the global object enables calling global functions and accessing global variables.

### Phase 3: Host Interop (Medium Priority)

Enable exposing Rust objects and functions to JavaScript.

#### HostFunction

```rust
pub type HostFunction = Box<dyn Fn(&mut JSRuntime, &JSValue, &[JSValue]) -> Result<JSValue, String>>;

impl JSFunction {
    pub fn create_from_host_function(
        runtime: &mut JSRuntime,
        name: &str,
        param_count: usize,
        func: HostFunction
    ) -> Result<JSFunction, String>;
}

impl JSObject {
    pub fn set_property_function(
        &mut self,
        runtime: &mut JSRuntime,
        name: &str,
        func: HostFunction
    ) -> Result<(), String>;
}
```

**Why:** Enables calling Rust code from JavaScript.

#### HostObject

```rust
pub trait HostObject: Send {
    fn get(&self, runtime: &mut JSRuntime, name: &str) -> Result<JSValue, String>;
    fn set(&mut self, runtime: &mut JSRuntime, name: &str, value: JSValue) -> Result<(), String>;
    fn get_property_names(&self, runtime: &mut JSRuntime) -> Vec<String>;
}

impl JSObject {
    pub fn create_from_host_object(
        runtime: &mut JSRuntime,
        host_object: Box<dyn HostObject>
    ) -> Result<JSObject, String>;
}
```

**Why:** Enables exposing Rust objects with custom behavior to JavaScript.

### Phase 4: Advanced JSI Features (Medium Priority)

#### Buffers and PreparedJavaScript

```rust
pub struct JSBuffer {
    data: Vec<u8>,
}

pub struct PreparedJavaScript {
    inner: cxx::SharedPtr<ffi::JSIPreparedJavaScript>,
}

impl Runtime {
    pub fn prepare_javascript(&mut self, buffer: &JSBuffer, source_url: &str)
        -> Result<PreparedJavaScript, String>;
    pub fn evaluate_prepared_javascript(&mut self, prepared: &PreparedJavaScript)
        -> Result<JSValue, String>;
}
```

**Why:** Optimize repeated evaluation of the same code.

#### WeakObject

```rust
pub struct JSWeakObject {
    inner: cxx::UniquePtr<ffi::JSIWeakObject>,
}

impl JSWeakObject {
    pub fn new(runtime: &mut JSRuntime, object: &JSObject) -> JSWeakObject;
    pub fn lock(&self, runtime: &mut JSRuntime) -> JSValue;
}
```

**Why:** Avoid memory leaks from circular references.

#### ArrayBuffer

```rust
pub struct JSArrayBuffer {
    inner: cxx::UniquePtr<ffi::JSIArrayBuffer>,
}

impl JSArrayBuffer {
    pub fn new(runtime: &mut JSRuntime, size: usize) -> JSArrayBuffer;
    pub fn size(&self, runtime: &mut JSRuntime) -> usize;
    pub fn data(&mut self, runtime: &mut JSRuntime) -> &mut [u8];
}
```

**Why:** Efficient binary data transfer between Rust and JavaScript.

### Phase 5: Hermes-Specific Features (Medium Priority)

#### Profiling

```rust
impl Runtime {
    pub fn enable_sampling_profiler(&mut self, freq_hz: f64);
    pub fn disable_sampling_profiler(&mut self);
    pub fn dump_sampled_trace_to_file(&mut self, filename: &str) -> Result<(), String>;
    pub fn dump_sampled_trace_to_stream(&mut self, stream: &mut dyn std::io::Write)
        -> Result<(), String>;
}
```

**Why:** Performance profiling of JavaScript code.

#### Unique IDs

```rust
impl JSObject {
    pub fn get_unique_id(&self, runtime: &mut JSRuntime) -> u64;
}

impl JSString {
    pub fn get_unique_id(&self, runtime: &mut JSRuntime) -> u64;
}

impl Runtime {
    pub fn get_object_for_id(&mut self, id: u64) -> Option<JSValue>;
}
```

**Why:** Track JavaScript objects across the C++/Rust boundary.

#### Execution Control

```rust
impl Runtime {
    pub fn async_trigger_timeout(&mut self);
    pub fn watch_time_limit(&mut self, timeout_ms: u32);
    pub fn unwatch_time_limit(&mut self);
}
```

**Why:** Interrupt long-running JavaScript code.

#### Source Maps

```rust
impl Runtime {
    pub fn evaluate_javascript_with_source_map(
        &mut self,
        source: &[u8],
        source_map: &[u8],
        source_url: &str
    ) -> Result<JSValue, String>;
}
```

**Why:** Better error messages and debugging.

### Phase 6: Low Priority Features

#### Microtasks

```rust
impl Runtime {
    pub fn queue_microtask(&mut self, callback: JSFunction);
    pub fn drain_microtasks(&mut self, max_hints: Option<i32>) -> bool;
}
```

#### Symbols

```rust
pub struct JSSymbol {
    inner: cxx::UniquePtr<ffi::JSISymbol>,
}

impl JSSymbol {
    // Create symbol
    pub fn new(runtime: &mut JSRuntime, description: &str) -> JSSymbol;

    // Compare
    pub fn strict_equals(runtime: &mut JSRuntime, a: &JSSymbol, b: &JSSymbol) -> bool;

    // Convert to string
    pub fn to_string(&self, runtime: &mut JSRuntime) -> String;
}
```

#### Serialization (JSI_UNSTABLE)

```rust
pub struct JSSerialized {
    inner: cxx::SharedPtr<ffi::JSISerialized>,
}

impl Runtime {
    pub fn serialize(&mut self, value: &JSValue) -> Result<JSSerialized, String>;
    pub fn deserialize(&mut self, serialized: &JSSerialized) -> Result<JSValue, String>;
}
```

#### Advanced Hermes Features

- Debugger integration
- GC execution trace
- I/O tracking
- Code coverage profiler
- Segment loading (for Metro bundler)
- Static Hermes (SHUnit) support
- Thread-safe runtime wrapper

---

## Summary

### Current Coverage

- ✅ **Runtime creation and configuration** - Fully implemented
- ✅ **Basic code evaluation** - eval, eval_with_result, bytecode
- ✅ **Value type checking** - All JSValue type checks
- ✅ **Basic object/array/string creation** - Factory methods only
- ❌ **Value extraction** - Can't get data from JSValue
- ❌ **Property access** - Can't read/write object properties
- ❌ **Function calls** - Can't call JavaScript functions
- ❌ **Host interop** - Can't expose Rust to JavaScript
- ❌ **Advanced features** - No buffers, weak refs, array buffers, etc.

### Prioritized Implementation Order

1. **Phase 1** (Critical): Value conversions, property access, array operations, string utilities
2. **Phase 2** (Critical): Function calls, global object access
3. **Phase 3** (Important): HostFunction, HostObject
4. **Phase 4** (Nice to have): Buffers, PreparedJavaScript, WeakObject, ArrayBuffer
5. **Phase 5** (Nice to have): Hermes-specific profiling, debugging, execution control
6. **Phase 6** (Low priority): Microtasks, Symbols, Serialization, advanced features

The first two phases are essential for basic JavaScript interop. Without them, the runtime can only evaluate code but cannot meaningfully interact with JavaScript objects or call functions.
