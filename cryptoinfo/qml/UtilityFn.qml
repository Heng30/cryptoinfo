import QtQuick 2.15

QtObject {
    function toBillion(num) {
        return (num / (1000 * 1000 * 1000)).toFixed(2) + "B";
    }

    function toMillion(num) {
        return (num / (1000 * 1000)).toFixed(2) + "M";
    }

    function asBillion(num, fixed) {
        return Number((num / (1000 * 1000 * 1000)).toFixed(fixed));
    }

    function billionAsNum(num) {
        return Number(num) * 1000 * 1000 * 1000;
    }

    function millionAsNum(num) {
        return Number(num) * 1000 * 1000;
    }

    function asMillion(num, fixed) {
        return Number((num / (1000 * 1000)).toFixed(fixed));
    }

    function asMillionOrBillion(num, fixed) {
        if (Number(num) > 1000 * 1000 * 1000)
            return asBillion(num, fixed);

        return asMillion(num, fixed);
    }

    function isAsBillion(num) {
        if (Number(num) > 1000 * 1000 * 1000)
            return true;

        return false;
    }

    function toFixedPrice(num) {
        const billion = 1000 * 1000 * 1000;
        const million = 1000 * 1000;
        if (num >= billion)
            return toBillion(num);
        else if (num >= million)
            return toMillion(num);
        else if (num >= 1000)
            return num.toFixed(0);
        else
            return num.toFixed(2);
    }

    function toPercentString(num) {
        return num.toFixed(2) + "%";
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

        if (num > 60 * 60)
            return seconds2Hours(num) + "h";
        else if (num > 60)
            return seconds2minus(num) + "m"
        else if (num > 1)
            return num + "s";
        else
            return seconds2milliseconds(num) + "ms"
    }

    function paddingSpace(num) {
        return String(" ").repeat(Number(num));
    }

}
