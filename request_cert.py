import time

import socketio
import tomli

cluster_config = tomli.load(open("config.toml", "rb"))

sio = socketio.Client()

center = "https://openbmclapi.bangbang93.com"

if __name__ == "__main__":
    
    cluster_id = cluster_config["cluster_id"]
    cluster_secret = cluster_config["cluster_secret"]
    """
        this.socket = connect(this.prefixUrl, {
      transports: ['websocket'],
      query: {
        clusterId: this.clusterId,
        clusterSecret: this.clusterSecret,
      },
    })
    """

    connect_url = f"{center}?clusterId={cluster_id}&clusterSecret={cluster_secret}"
    sio.connect(connect_url, transports="websocket")
    print("connected")
    time.sleep(1)

    """
    const cert = await new Promise<{cert: string; key: string}>((resolve, reject) => {
      this.socket?.emit('request-cert', ([err, cert]: [unknown, {cert: string; key: string}]) => {
        if (err) return reject(err)
        resolve(cert)
      })
    })
    """
    # sio.emit("request-cert",)
    sio.emit("request-cert", callback=lambda *args: print(args))

    time.sleep(10)
