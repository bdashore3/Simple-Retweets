# Simple Retweets

A retweet bot written in pure Rust. Currently uses API polling to bulk retweet.

This bot has a [caching system](https://github.com/bdashore3/Simple-Retweets/commit/f084c1380bfe96f4ed79702a508bbebdf9a603cd#diff-4526c02ea22c9a70285696db52fc53e6) that prevents API rate limiting by storing already retweeted tweet ids and ignoring the retweet command if the cache contains the tweet.

## Preparation

### Developer Account
This bot requires a twitter developer account. Apply for one [here](https://developer.twitter.com/en). The developer account does NOT have to be on the account you're sending retweets on!

Once you have an account, create a new app and give it Read/Write permissions. 

Finally, grab the consumer key and token to be used later.

## Installation

### Downloading the bot
Download the latest binary from the [releases](https://github.com/bdashore3/Simple-Retweets/releases) and use FTP or SCP to push the file to your server!
(Alternatively, you can use wget or curl to download the binary directly to the server itself).

### Configuration
Then, copy **info.sample.json** to **info.json** in the project directory. From there, add all the following credentials from twitter.
```
- consumer_key
- consumer_secret
```

There are also optional parameters (Measure the amount of mentions the account gets in every x minutes. The goal is to capture as many new mentions as possible) :
```
- rt_delay: Delay between api checks (Default: 3 minutes)
- page_size: Amount of fetched mentions per check (Default: 5)
```

### Finally:
Once you're done, type the following command in the terminal inside the binary directory:
```
./simple-retweets info.json
```

## Running in a server

The included systemd service is HIGHLY RECOMMENDED to run this bot in a server. Running in interactive mode is not advised. 

The service assumes you're under a user called `simplerts` and the binary is inside a directory called `Simple-Retweets` with the binary name being `simple-retweets`.

Copy the simplerts.service file into /etc/systemd/system/simplerts.service. Then, run these commands
```
sudo systemctl reload-daemon
sudo systemctl enable simplerts.service
sudo systemctl start simplerts.service
```

Check with:
```
sudo systemctl status simplerts.service
sudo journalctl -u simplerts -f
```

## Removing the bot

It's easy! All you have to do is delete the bot directory and the systemd file from `/etc/systemd/system/simplerts.service`

# Developers and Permissions

Currently, this bot is allowed for use outside of the developer's server. I try to make the comments as detailed as possible, but if you don't understand something, please contact me via the Discord server! I'm always happy to talk!

Creator/Developer: Brian Dashore

Developer Twitter: [@kingbri1st](https://twitter.com/kingbri1st)

Developer Discord: kingbri#6666

Join the support discord here (get the king-updates role to access the channel): [https://discord.gg/pswt7by](https://discord.gg/pswt7by)