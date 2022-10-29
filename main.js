const os = require('os'); 
const path = require('path');

const platform = os.platform(); 
let dir = '';
switch(platform) { 
    case 'aix': {
        throw new Error("IBM AIX platform not supported");
    }; 
    case 'android': {
        throw new Error("Android platform is not supported");
    };
    case 'darwin': {
        // console.log("Darwin platfrom(MacOS, IOS etc)"); 
        dir = 'macos';
        break; 
    }
    case 'freebsd': {
        throw new Error("FreeBSD Platform is not supported");
    }
    case 'linux': {
        // console.log("Linux Platform"); 
        dir = 'linux';
        break;
    }
    case 'openbsd': {
        throw new Error("OpenBSD Platform is not supported");
    }
    case 'sunos': {
        throw new Error("SunOS platform is not supported");
    }
    case 'win32': {
        // console.log("windows platform"); 
        dir = 'win32';
        break;
    }     
    default: {
        // console.log("unknown platform")
        throw new Error("Unknown platform");
    }; 
}

const nodePath = path.join(__dirname, '/platform/', dir, '/index.node');

module.exports = require(nodePath);
