import Vue from "vue";

export interface ErrorState {
  [name: string]: boolean;
}

export abstract class VueWithErrorCatcher<T extends ErrorState> extends Vue {
  protected errorStates: T;

  // TODO: We need the error states ahead of time because otherwise they won't be reactive.
  // This can possibly be fixed with Vue 3 by manually creating reactive references.
  protected constructor(errorStates: T) {
    super();
    this.errorStates = errorStates;
  }

  get hasError(): boolean {
    return Object.values(this.errorStates).some(err => err === true);
  }

  protected setError<K extends keyof T>(state: K, value: boolean) {
    (this.errorStates as any)[state] = value;
  }

  protected setErrorAndPropagate<K extends keyof T>(state: K, value: boolean) {
    this.setError(state, value);
    this.$emit("has-error", this.hasError);
  }
}

export default abstract class VueWithError<T> extends Vue {
  protected error: T | null = null;

  protected setError(error: T | null) {
    if (this.error === error) return;

    this.error = error;
    this.$emit("has-error", this.error !== null);
  }
}
