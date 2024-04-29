// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import getNumLevels from "./levels/getNumLevels";
import {getAddForm, trim, addLevel} from "./setupTests";
import setup from "../../../../../components/error/setUpTests";

document.body.innerHTML = getAddForm();
document.body.appendChild(setup());

jest.useFakeTimers();

it("addLevelButton works", () => {
  expect(getNumLevels()).toBe(1);
  // add a level
  addLevel(2, 4);
  expect(getNumLevels()).toBe(2);


  
  // add second level
  addLevel(4, 9);
  expect(getNumLevels()).toBe(3);

  const a = document.body.innerHTML;


  expect(trim(a)).toBe(trim(finalHtml()));

  // try to add duplicate level
  addLevel(2, 4);
  expect(getNumLevels()).toBe(3);

  // try to add negative parameters
  addLevel(-4, -9);
  expect(getNumLevels()).toBe(3);
});

const finalHtml = () => {
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
        name="visitor2"
        
        id="visitor2"
      >
    </label>

    <label class="sitekey-form__level-label" for="difficulty2">
      Difficulty
      <input
        type="number"
        name="difficulty2"
        class="sitekey-form__level-input"
        
        id="difficulty2"
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
<div id="err__container">
  </div>
  `;
};
