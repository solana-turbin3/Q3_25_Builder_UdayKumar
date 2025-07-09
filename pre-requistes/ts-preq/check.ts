import bs58 from "bs58";

// paste your Base58 private key here:
const base58Key = "D6o5ZhgPNi57UHxLeDphhZj8RCYgMGwjm8NiEUEzFst1jTYhRPpcvvzyz4Tq76pfoYUweTS6kmyHLa1JdMXk9qa";  

// decode to a Buffer, then turn into an array of bytes
const secretKeyBytes = Uint8Array.from(bs58.decode(base58Key));

console.log(JSON.stringify(Array.from(secretKeyBytes)));