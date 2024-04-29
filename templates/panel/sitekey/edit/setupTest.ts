// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

export const EDIT_FORM = `
<form class="sitekey-form" action="/api/v1/mcaptcha/levels/update" method="post" >
  <h1 class="form__title">
    Sitekey: test
  </h1>
  <label class="sitekey-form__label" for="description">
    Description
    <input
      class="sitekey-form__input"
      type="text"
      name="description"
      id="description"
      required=""
      value="test"
    />
  </label>
  <label class="sitekey-form__label" for="duration">
    Cooldown Duration(in seconds)
    <input
      class="sitekey-form__input"
      type="number"
      name="duration"
      id="duration"
      min="0"
      required=""
      value="30"
    />
  </label>

  <fieldset class="sitekey__level-container" id="level-group-1">
    <legend class="sitekey__level-title">
      Level 1
    </legend>

    <label class="sitekey-form__level-label" for="visitor1"
      >Visitor
      <input
        class="sitekey-form__level-input"
        type="number"
        name="visitor1"
        value="4"
        id="visitor1"
      />
    </label>
    <label class="sitekey-form__level-label" for="difficulty1">
      Difficulty
      <input
        type="number"
        id="difficulty1"
        class="sitekey-form__level-input"
        value="5"
      />
    </label>
    <label class="sitekey-form__level-label--hidden" for="remove1">
      RemoveLevel
      <input
        class="sitekey-form__level-remove-level-button"
        type="button"
        id="remove-level1"
        value="x"
    /></label>
  </fieldset>

  <fieldset class="sitekey__level-container" id="level-group-2">
    <legend class="sitekey__level-title">
      Level 2
    </legend>
    <label class="sitekey-form__level-label" for="visitor2"
      >Visitor
      <input
        class="sitekey-form__level-input"
        type="number"
        name="visitor2"
        id="visitor2"
      />
    </label>

    <label class="sitekey-form__level-label" for="difficulty2">
      Difficulty
      <input
        type="number"
        name="difficulty2"
        class="sitekey-form__level-input"
        id="difficulty2"
      />
    </label>
    <label class="sitekey-form__level-label--hidden" for="add">
      <span class="sitekey-form__add-level-btn-spacer">Add level</span>
      <input
        class="sitekey-form__level-add-level-button"
        type="button"
        name="add"
        id="add"
        value="Add"
      />
    </label>
  </fieldset>
  <button
    data-sitekey="9FGkkukDRFDk7FgJmjXKxeHjFHUcxNez"
    class="sitekey-form__submit"
    type="submit"
  >
    Submit
  </button>
</form>
<div id="err__container"></div>
`;
