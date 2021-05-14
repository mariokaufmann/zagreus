function setText(id: string, oldText: string, newText: string) {
    cy.get(`#${id}`).should('have.text', oldText);
    cy.request('POST', '/api/template/e2e-template/data/text', {
        id,
        text: newText,
    });
    cy.get(`#${id}`).should('have.text', newText);
}

describe('E2E test', () => {

    it('should pass test scenario', () => {
        cy.visit('/static/template/e2e-template')

        cy.get('#zagreus-svg-container').should('have.class', 'zagreus-hidden');
        // wait until zagreus runtime has initialized
        cy.get('#zagreus-svg-container').should('not.have.class', 'zagreus-hidden');

        // show animation
        cy.get('#LowerThird').should('not.be.visible');
        cy.request('POST', '/api/template/e2e-template/data/animation/lowerthirdshow');
        cy.get('#LowerThird').should('be.visible');

        // text manipulation
        setText('LowerThirdTitle', 'Title', 'New title');
        setText('LowerThirdSubtitle', 'Subtitle', 'New subtitle');
        setText('LowerThirdRightAlignedText', 'Right 12', 'Long text 11');

        // CSS class manipulation
        cy.get('#LowerThirdRightAlignedText').should('be.visible');
        cy.request('POST', '/api/template/e2e-template/data/class/add', {
            id: 'LowerThirdRightAlignedText',
            'class': 'hidden',
        });
        cy.get('#LowerThirdRightAlignedText').should('not.be.visible');
        cy.request('POST', '/api/template/e2e-template/data/class/remove', {
            id: 'LowerThirdRightAlignedText',
            'class': 'hidden',
        });
        cy.get('#LowerThirdRightAlignedText').should('be.visible');

        // image manipulation
        cy.get('#LowerThirdLogo').should('not.have.attr', 'href');
        cy.request('POST', '/api/template/e2e-template/data/image', {
            id: 'LowerThirdLogo',
            asset: 'dragon.png',
        });
        cy.get('#LowerThirdLogo').should('have.attr', 'href').and('equal', 'assets/dragon.png');
    })

});
