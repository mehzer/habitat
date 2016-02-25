import * as util from "./util";

describe("util", () => {
    describe("packageString", () => {
        describe("with a fully qualified identifier", () => {
            it("returns the string", () => {
                expect(util.packageString({
                    origin: "testderiv",
                    name: "testname",
                    version: "1.0.0",
                    release: "197001010000",
                })
                ).to.eq("testderiv/testname/1.0.0/197001010000");
            });
        });

        describe("with a missing parts", () => {
            it("returns the partial string", () => {
                expect(util.packageString({
                    origin: "testderiv",
                    name: "testname",
                })
                ).to.eq("testderiv/testname");
            });
        });
    });
});
