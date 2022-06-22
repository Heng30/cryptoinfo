function toBillion(num) {
  return (Number(num) / (1000 * 1000 * 1000)).toFixed(2) + 'B';
}

function toMillion(num) {
  return (Number(num) / (1000 * 1000)).toFixed(2) + 'M';
}

function asBillion(num, fixed) {
  return Number((Number(num) / (1000 * 1000 * 1000)).toFixed(fixed));
}

function billionAsNum(num) {
  return Number(num) * 1000 * 1000 * 1000;
}

function millionAsNum(num) {
  return Number(num) * 1000 * 1000;
}

function asMillion(num, fixed) {
  return Number((Number(num) / (1000 * 1000)).toFixed(fixed));
}

function asMillionOrBillion(num, fixed) {
  if (Number(num) > 1000 * 1000 * 1000) return asBillion(num, fixed);

  return asMillion(num, fixed);
}

function isAsBillion(num) {
  if (Number(num) > 1000 * 1000 * 1000) return true;

  return false;
}

function toFixedPrice(num) {
  num = Number(num);
  var flag = num > 0;
  num = Math.abs(num);
  const billion = 1000 * 1000 * 1000;
  const million = 1000 * 1000;
  if (num >= billion) return flag ? toBillion(num) : -toBillion(num);
  else if (num >= million) return flag ? toMillion(num) : -toMillion(num);
  else if (num >= 1000) return flag ? num.toFixed(0) : -num.toFixed(0);
  else if (num >= 0.01) return flag ? num.toFixed(2) : -num.toFixed(2);
  else if (num >= 0.0001) return flag ? num.toFixed(4) : -num.toFixed(4);
  else return flag ? num.toFixed(6) : -num.toFixed(6);
}

function toPercentString(num) {
  return Number(num).toFixed(2) + '%';
}

function seconds2milliseconds(num) {
  return Number(num) * 1000;
}

function minus2seconds(num) {
  return Number(num) * 60;
}

function hours2seconds(num) {
  return Number(num) * 3600;
}

function seconds2minus(num) {
  return Number(num) / 60;
}

function seconds2Hours(num) {
  return Number(num) / 3600;
}

function seconds2FixedTime(num) {
  var num = Number(num);
  if (num > 60 * 60) return seconds2Hours(num) + 'h';
  else if (num > 60) return seconds2minus(num) + 'm';
  else if (num > 1) return num + 's';
  else return seconds2milliseconds(num) + 'ms';
}

function paddingSpace(num) {
  return String(' ').repeat(Number(num));
}

function chttp(method, url, okCB, errorCB, timeoutCB) {
  if (!okCB && !errorCB && !timeoutCB) return;

  const Http = new XMLHttpRequest();
  Http.open(method, url);
  if (errorCB) Http.onerror = errorCB;
  if (timeoutCB) Http.ontimeout = timeoutCB;
  if (okCB) {
    Http.onreadystatechange = function () {
      if (Http.readyState !== 4 || Http.status !== 200) return;

      const text = Http.responseText;
      if (text.length <= 0) return;
      okCB(text);
    };
  }

  Http.send();
}
