const topbarApp = Vue.createApp({
  data() {
    return {
      checkedPanel: null,
      updateTime: null,
      fearGreedToday: 0,
      fearGreadYesterday: 0,
      btcNextHalvingdaysleft: -1,
      totalMarketCap: 0,
      total24hVolumn: 0,
      btcPercentOfMarketCap: 0,
      upCoinPricePercent: 0,
    };
  },
  methods: {
    setUpdateTime(panel, time) {
      if (panel === this.checkedPanel) this.updateTime = time;
      else this.updateTime = null;
    },
    _loadItems() {
      this._loadFearGreed();
      this._loadMarket();
      this._loadBtcNextHalvingDaysLeft();
    },

    _loadFearGreed() {
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
              root.fearGreedToday = data[i].value;
            } else {
              root.fearGreadYesterday = data[i].value;
            }
          }
        } catch (e) {
          console.log(e);
        }
      });
    },

    _loadBtcNextHalvingDaysLeft() {
      var root = this;
      const url = serverUrl + '/apiv1/coin/btc-next-halving-day-left';
      chttp('GET', url, function (text) {
        try {
          var data = JSON.parse(text);
          if (!data.days) return;
          root.btcNextHalvingdaysleft = data.days;
        } catch (e) {
          console.log(e);
        }
      });
    },

    _loadMarket() {
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
}).mount('#top-bar');
