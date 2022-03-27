import QtQuick 2.15

QtObject {
    function toBillion(num) {
        return (num / (1000 * 1000 * 1000)).toFixed(2) + "B";
    }

    function toMillion(num) {
        return (num / (1000 * 1000)).toFixed(2) + "M";
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

}
