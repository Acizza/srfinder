interface Ok<T> {
  kind: "ok";
  value: T;
}

interface Err<T> {
  kind: "err";
  value: T;
}

type Result<T, E> = Ok<T> | Err<E>;

namespace Result {
  export function ok<T>(value: T): Ok<T> {
    return { kind: "ok", value };
  }

  export function err<T>(value: T): Err<T> {
    return { kind: "err", value };
  }
}

export default Result;
