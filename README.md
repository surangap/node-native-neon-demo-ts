# node-native-neon-demo-ts
Demo - neon generates the node native module via FFI from rust to NAPI. Then the native module functionalities are accessed from typescript. 

1. clone the repo
2. run `npm install` in project root.
3. go to `ts` directory and run `npm install` again. Then run `npm run build` to compile ts files in the same directory.
4. you should see a `dist` directory now, to run the demo, execute `node dist/tester.js` 
