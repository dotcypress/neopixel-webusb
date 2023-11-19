<script>
  import 'hack';
  import 'hack/dist/dark.css';
  import { ZapIcon, SunIcon, GithubIcon } from 'svelte-feather-icons';
  import { onMount } from 'svelte';

  // @ts-ignore
  let usb = navigator.usb;
  let device;
  let color = '';

  $: color, sendColor();

  onMount(async () => {
    usb.addEventListener('connect', ({ device }) => {
      open(device);
    });

    usb.addEventListener('disconnect', (ev) => {
      if (ev.device === device) {
        device?.close();
        device = null;
      }
    });

    let knownDevices = await usb?.getDevices();
    for (const device of knownDevices) {
      if (!device.opened) {
        await open(device);
        break;
      }
    }
  });

  async function connect() {
    let usbDevice = await usb.requestDevice({
      filters: [{ vendorId: 0x1209, productId: 0xb420 }]
    });
    if (!usbDevice.opened) {
      open(usbDevice);
    }
  }

  async function open(usbDevice) {
    device?.close();
    device = null;

    device = usbDevice;
    await device.open();
    await device.claimInterface(0);
    let deviceColor = await device.controlTransferIn(
      {
        requestType: 'vendor',
        recipient: 'device',
        request: 0x01,
        value: 0x000,
        index: 0x0000
      },
      3
    );

    let r = deviceColor.data.getUint8(0).toString(16).padStart(2, '0');
    let g = deviceColor.data.getUint8(1).toString(16).padStart(2, '0');
    let b = deviceColor.data.getUint8(2).toString(16).padStart(2, '0');
    color = `#${r}${g}${b}`;
  }

  async function sendColor() {
    let rgb = parseInt(color.replaceAll('#', ''), 16);
    await device?.controlTransferOut(
      {
        requestType: 'vendor',
        recipient: 'device',
        request: 0x01,
        value: 0x0000,
        index: 0x0000
      },
      new Uint8Array([rgb >> 16, rgb >> 8, rgb])
    );
  }
</script>

<div class="container">
  <br />
  <div class="grid">
    <div class="cell -8of12 -left">
      <div class="grid" style="margin-top: 8px;">
        <div id="sun" class="cell -1of12">
          <SunIcon />
        </div>
        <div class="cell -3of12" style="line-height: 24px;">NeoPixel WebUSB</div>
      </div>
    </div>
    <div class="cell -4of12 -right">
      <div class="pull-right">
        {#if usb && !device}
          <button class="btn btn-warning" title="Connect" on:click={connect}>
            <ZapIcon />
          </button>
        {/if}
        <a class="btn btn-info btn-ghost" href="https://github.com/dotcypress/neopixel-webusb">
          <GithubIcon />
        </a>
      </div>
    </div>
  </div>
  <br />
  {#if device}
    <div class="grid -center">Tap to change NeoPixel color</div>
    <div class="grid -center">
      <div id="color-picker" style="background-color: {color};">
        <input type="color" bind:value={color} />
      </div>
    </div>
  {:else if !usb}
    <div class="alert alert-error">
      The <a href="https://caniuse.com/?search=web-usb">WebUSB API</a> is not supported by your browser
    </div>
  {/if}
</div>

<style>
  @keyframes colorRotate {
    from {
      color: #e81416;
    }
    20% {
      color: #ffa500;
    }
    40% {
      color: #faeb36;
    }
    60% {
      color: #79c314;
    }
    80% {
      color: #487de7;
    }
    100% {
      color: #70369d;
    }
  }
  input[type='color'] {
    opacity: 0;
    width: 234px;
    height: 100px;
  }
  #sun {
    animation: colorRotate 3s linear 0s infinite;
    animation-direction: alternate;
  }
  #color-picker {
    border: 1px solid #ccc;
    border-radius: 72px;
  }
</style>
