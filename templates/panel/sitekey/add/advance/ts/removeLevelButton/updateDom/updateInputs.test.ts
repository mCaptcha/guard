// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import {getAddForm, trim} from "../../setupTests";
import updateInputs from "./updateInputs";
import CONST from "../../const";

import log from "../../../../../../../logger";
import {MODE} from "../../../../../../../logger";

import {setupAddlevels} from "./setupTests";

document.body.innerHTML = getAddForm();

log.setMode(MODE.none);

it("updateInputs works", () => {
  setupAddlevels();
  // removing level  2
  const level = 2;
  const levelGroup = document.querySelector(
    `#${CONST.LEVEL_FIELDSET_ID_WITHOUT_LEVEL}${level}`,
  );

  const newLevel = 20;

  updateInputs(levelGroup, newLevel);

  const inputs = <NodeListOf<HTMLInputElement>>(
    levelGroup.querySelectorAll(`.${CONST.LEVEL_INPUT_CLASS}`)
  );
  inputs.forEach(input => {
    if (input.id.includes(CONST.VISITOR_WITHOUT_LEVEL)) {
      expect(input.id).toBe(`${CONST.VISITOR_WITHOUT_LEVEL}${newLevel}`);
      console.log("checking visitor");
    } else {
      //    if (input.id.includes(CONST.DIFFICULTY_WITHOUT_LEVEL)) {
      console.log("checking difficulty");
      expect(input.id).toBe(`${CONST.DIFFICULTY_WITHOUT_LEVEL}${newLevel}`);
    }
  });

  expect(trim(document.body.innerHTML)).toBe(trim(update()));
});

/** get initial form to test remove button functionality */
export const update = (): string => {
  return `
<form class="sitekey-form" action="/api/v1/mcaptcha/levels/add" method="post">
  <h1 class="form__title">
    Add Sitekey
  </h1>
  <label class="sitekey-form__label" for="description">
    Description
    <input
      class="sitekey-form__input"
      type="text"
      name="description"
      id="description"
      required=""
      
    >
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
    >
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
        
        id="visitor1"
      >
    </label>

    <label class="sitekey-form__level-label" for="difficulty1">
      Difficulty
      <input
        type="number"
        name="difficulty1"
        class="sitekey-form__level-input"
        
        id="difficulty1"
      >
    </label>
    <label class="sitekey-form__level-label--hidden" for="remove-level1">
      Remove Level
      <input
        class="sitekey-form__level-remove-level-button"
        type="button"
        name="remove-level1"
        id="remove-level1"
        value="x"
      >
    </label>
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
        name="visitor20"
        
        id="visitor20"
      >
    </label>

    <label class="sitekey-form__level-label" for="difficulty2">
      Difficulty
      <input
        type="number"
        name="difficulty20"
        class="sitekey-form__level-input"
        
        id="difficulty20"
      >
    </label>
    <label class="sitekey-form__level-label--hidden" for="remove-level2">
      Remove Level
      <input
        class="sitekey-form__level-remove-level-button"
        type="button"
        name="remove-level2"
        id="remove-level2"
        value="x"
      >
    </label>
  </fieldset>
  <fieldset class="sitekey__level-container" id="level-group-3">
    <legend class="sitekey__level-title">
      Level 3
    </legend>
    <label class="sitekey-form__level-label" for="visitor3"
      >Visitor
      <input
        class="sitekey-form__level-input"
        type="number"
        name="visitor3"
        
        id="visitor3"
      >
    </label>

    <label class="sitekey-form__level-label" for="difficulty3">
      Difficulty
      <input
        type="number"
        name="difficulty3"
        class="sitekey-form__level-input"
        
        id="difficulty3"
      >
    </label>
    <label class="sitekey-form__level-label--hidden" for="remove-level3">
      Remove Level
      <input
        class="sitekey-form__level-remove-level-button"
        type="button"
        name="remove-level3"
        id="remove-level3"
        value="x"
      >
    </label>
  </fieldset>

  <fieldset class="sitekey__level-container" id="level-group-4">
    <legend class="sitekey__level-title">
      Level 4
    </legend>
    <label class="sitekey-form__level-label" for="visitor4"
      >Visitor
      <input
        class="sitekey-form__level-input"
        type="number"
        name="visitor4"
        
        id="visitor4"
      >
    </label>

    <label class="sitekey-form__level-label" for="difficulty4">
      Difficulty
      <input
        type="number"
        name="difficulty4"
        class="sitekey-form__level-input"
        
        id="difficulty4"
      >
    </label>
    <label class="sitekey-form__level-label--hidden" for="add">
      Add level
      <input
        class="sitekey-form__level-add-level-button"
        type="button"
        name="add"
        id="add"
        value="Add"
      >
    </label>
  </fieldset>

  <button class="sitekey-form__submit" type="submit">Submit</button>
</form>
`;
};
