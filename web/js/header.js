const updateFearGreed = {
  data() {
    return {
      today: 0,
      yesterday: 0,
    };
  },

  methods: {
    _loadItems() {
      var root = this;
      const url = serverUrl + '/apiv1/fear-greed';
      chttp('GET', url, function (text) {
        try {
          var data = JSON.parse(text);
          if (!data.data) return;
          data = data.data;
          if (!Array.isArray(data)) return;
          for (var i = 0; i < data.length; i++) {
            if (i === 0) {
              root.today = data[i].value;
            } else {
              root.yesterday = data[i].value;
            }
          }
        } catch (e) {
          console.log(e);
        }
      });
    },
  },

  mounted() {
    this._loadItems();
    setInterval(() => {
      this._loadItems();
    }, 60000);
  },
};

Vue.createApp(updateFearGreed).mount('#fear-greed');

const updateMarket = {
  data() {
    return {
      totalMarketCap: 0,
      total24hVolumn: 0,
      btcPercentOfMarketCap: 0,
    };
  },

  methods: {
    _loadItems() {
      var root = this;
      const url = serverUrl + '/apiv1/market';
      chttp('GET', url, function (text) {
        try {
          var data = JSON.parse(text);
          if (data.total_market_cap_usd)
            root.totalMarketCap = toFixedPrice(data.total_market_cap_usd);
          if (data.total_24h_volume_usd)
            root.total24hVolumn = toFixedPrice(data.total_24h_volume_usd);
          if (data.bitcoin_percentage_of_market_cap)
            root.btcPercentOfMarketCap = Number(
              data.bitcoin_percentage_of_market_cap
            ).toFixed(2);
        } catch (e) {
          console.log(e);
        }
      });
    },
  },

  mounted() {
    this._loadItems();
    setInterval(() => {
      this._loadItems();
    }, 60000);
  },
};

Vue.createApp(updateMarket).mount('#total-market-cap');
Vue.createApp(updateMarket).mount('#total-24h-volumn');
Vue.createApp(updateMarket).mount('#btc-percent-of-market-cap');

const updateBTCNextHalving = {
  data() {
    return {
      days: -1,
    };
  },

  methods: {
    _loadItems() {
      var root = this;
      const url = serverUrl + '/apiv1/coin/btc-next-halving-day-left';
      chttp('GET', url, function (text) {
        try {
          var data = JSON.parse(text);
          if (!data.days) return;
          root.days = data.days;
        } catch (e) {
          console.log(e);
        }
      });
    },
  },

  mounted() {
    this._loadItems();
    setInterval(() => {
      this._loadItems();
    }, 60000);
  },
};

Vue.createApp(updateBTCNextHalving).mount('#btc-next-halving-days-left');
