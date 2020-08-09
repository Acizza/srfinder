<style>
  .spinner {
    margin: auto;
  }

  .spinner > div {
    width: 18px;
    height: 18px;
    background-color: var(--text-color);
    border-radius: 100%;
    display: inline-block;
    animation: sk-bouncedelay 1.4s infinite ease-in-out both;
  }

  .spinner .bounce1 {
    animation-delay: -0.32s;
  }

  .spinner .bounce2 {
    animation-delay: -0.16s;
  }

  @keyframes sk-bouncedelay {
    0%,
    80%,
    100% {
      transform: scale(0);
    }
    40% {
      transform: scale(1);
    }
  }
</style>

<!-- From https://github.com/tobiasahlin/SpinKit -->
<div class="spinner">
  <div class="bounce1" />
  <div class="bounce2" />
  <div class="bounce3" />
</div>
