import {
  DiscoverAll,
  DiscoverPool
} from "./plugin.ts";

type IDiscoverParams = {
  host: string;
  delay: number;
  onDeviceFound: Function;
};

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

  async start() {
    // register in rust
    await DiscoverAll({
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

setInterval(() => {
  console.log("discov")
  console.log(DiscoverPool())
}, 3000)

await mdns.start();
console.log("im here")
console.log(mdns.actions);

// stop after 5 sec
setTimeout(() => {
  mdns.stop();
}, 500000);

setTimeout(() => {
  console.log(mdns);
}, 1000);



console.log("waiting....")