const { execFileSync } = require("child_process");

(async () => {
    console.log("🎄 Advent of Code 2018\n");
    for (let d = 1; d <= 9; d += 1) {
        console.log(`> Day ${d}`);
        const day = d >= 10 ? d : `0${d}`;
        execFileSync("node", [`src/day${day}.js`], {
            stdio: "inherit"
        });
        console.log();
    }
})();
