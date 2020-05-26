import {
  DiscoverAll,
} from "./plugin.ts";

export default class MDNS {
  browser: any;
  actions: Record<string, any> = {};
  host: string;
  delay: Number;
  constructor(host: string, checkDelay: Number) {
    this.host = host;
    this.delay = checkDelay;
  }

  on(action: string, fn: Function) {
    this.actions[action] = fn;
  }

  start() {
    // register in rust
    DiscoverAll({
      host: this.host,
      delay: this.delay,
    });
  }

  stop() {
    console.log("DESTROY");
  }
}

const mdns = new MDNS("_googlecast._tcp.local", 2);

mdns.on("serviceUp", (service: any) => {
  console.log(service);
});

mdns.on("serviceDown", (service: any) => {
  console.log(service);
});

mdns.start();
console.log(mdns.actions);

// stop after 5 sec
setTimeout(() => {
  mdns.stop();
}, 50000);

setTimeout(() => {
  console.log(mdns);
}, 1000);
