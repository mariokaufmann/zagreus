function getElement(name: string) {
  return cy.get(`[data-zag="${name}"]`);
}

function setText(name: string, oldText: string, newText: string) {
  getElement(name).should("have.text", oldText);
  cy.request("POST", "http://localhost:8080/api/instance/e2e/data/text", {
    id: name,
    text: newText,
  });
  getElement(name).should("have.text", newText);
}

describe("E2E test", () => {
  it("should pass test scenario", () => {
    cy.visit("/");

    getElement("zagreus-container").should("have.class", "zagreus-hidden");
    // wait until zagreus runtime has initialized
    getElement("zagreus-container").should("not.have.class", "zagreus-hidden");

    // show animation
    getElement("LowerThird").should("not.be.visible");
    cy.request(
      "POST",
      "http://localhost:8080/api/instance/e2e/data/animation/LowerThirdShow"
    );
    getElement("LowerThird").should("be.visible");

    // text manipulation
    setText("LowerThirdTitle", "Title", "New title");
    setText("LowerThirdSubtitle", "Subtitle", "New subtitle");
    setText("LowerThirdRightAlignedText", "Right 12", "Long text 11");

    // CSS class manipulation
    getElement("LowerThirdRightAlignedText").should("be.visible");
    cy.request(
      "POST",
      "http://localhost:8080/api/instance/e2e/data/class/add",
      {
        id: "LowerThirdRightAlignedText",
        class: "hidden",
      }
    );
    getElement("LowerThirdRightAlignedText").should("not.be.visible");
    cy.request(
      "POST",
      "http://localhost:8080/api/instance/e2e/data/class/remove",
      {
        id: "LowerThirdRightAlignedText",
        class: "hidden",
      }
    );
    getElement("LowerThirdRightAlignedText").should("be.visible");

    // image manipulation
    cy.request("POST", "http://localhost:8080/api/instance/e2e/data/image", {
      id: "LowerThirdLogo",
      asset: "dragon.png",
      assetSource: "template",
    });
    getElement("LowerThirdLogo")
      .should("have.attr", "src")
      .and("equal", "dragon.png");

    // upload dynamic asset
    cy.fixture("dog.svg", "binary").then((imageData) => {
      let blob = Cypress.Blob.binaryStringToBlob(imageData, "image/svg+xml");
      const data = new FormData();
      data.set("name", "dog.svg");
      data.set("file", blob);

      cy.request("POST", "http://localhost:8080/api/asset", data).then(() => {
        cy.request(
          "POST",
          "http://localhost:8080/api/instance/e2e/data/image",
          {
            id: "LowerThirdLogo",
            asset: "dog.svg",
            assetSource: "zagreus",
          }
        );
        getElement("LowerThirdLogo").should("be.visible");
        getElement("LowerThirdLogo")
          .should("have.attr", "src")
          .and("equal", "http://localhost:8080/assets/dog.svg");

        // TODO potentiall re-add this endpoint?
        // cy.request("http://localhost:8080/api/asset").then((response) => {
        //   let assets = response.body;
        //   expect(assets.length).to.equal(3);
        //   expect(assets).to.deep.contain({ name: "main.css" });
        //   expect(assets).to.deep.contain({ name: "dog.svg" });
        //   expect(assets).to.deep.contain({ name: "dragon.png" });
        // });
      });
    });
  });
});
