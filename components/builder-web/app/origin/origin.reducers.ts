// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import { List } from "immutable";
import * as actionTypes from "./origin.actions";
import initialState from "../initialState";
import { OriginRecord } from "../records/origin-record";
import {
    POPULATE_ORIGIN_MEMBERS,
    SET_ORIGIN_USER_INVITE_ERROR_MESSAGE
} from "./origin-page/origin-members-tab/origin-members-tab.actions";
import {
    POPULATE_ORIGIN_INTEGRATIONS
} from "./origin-page/origin-integrations-tab/origin-integrations-tab.actions";
import {
    POPULATE_ORIGIN_PUBLIC_KEYS,
    SET_ORIGIN_PRIVATE_KEY_UPLOAD_ERROR_MESSAGE,
    SET_ORIGIN_PUBLIC_KEY_UPLOAD_ERROR_MESSAGE
} from "./origin-page/origin-keys-tab/origin-keys-tab.actions";
import { SET_CURRENT_ORIGIN_CREATING_FLAG } from "./origin-create-page/origin-create-page.actions";
import { POPULATE_MY_ORIGIN_INVITATIONS } from "./origins-page/origins-page.actions";

export default function origins(state = initialState["origins"], action) {
    switch (action.type) {
        case actionTypes.POPULATE_MY_ORIGINS:
            if (action.error) {
                return state.setIn(["mine"], List()).
                    setIn(["ui", "mine", "errorMessage"], action.error.message).
                    setIn(["ui", "mine", "loading"], false);
            } else {
                return state.setIn(["mine"], List(action.payload.map(name =>
                    OriginRecord({ name })
                ))).setIn(["ui", "mine", "errorMessage"], undefined).
                    setIn(["ui", "mine", "loading"], false);
            }

        case actionTypes.SET_PACKAGE_COUNT_FOR_ORIGIN:
            if (action.payload.origin) {
                const record = state.getIn(["mine"]).find(value => value.get("name") === action.payload.origin);
                const index = state.getIn(["mine"]).indexOf(record);
                const newRecord = record.merge({packageCount: action.payload.unique_packages});
                return state.setIn(["mine", index], newRecord)
                    .setIn(["ui", "mine", "errorMessage"], undefined)
                    .setIn(["ui", "mine", "loading"], false);
            } else {
                return state;
            }


        case POPULATE_MY_ORIGIN_INVITATIONS:
            return state.setIn(["myInvitations"],
                List(action.payload));

        case actionTypes.POPULATE_ORIGIN_INVITATIONS:
            return state.setIn(["currentPendingInvitations"],
                List(action.payload));

        case POPULATE_ORIGIN_MEMBERS:
            return state.setIn(["currentMembers"],
                List(action.payload));

        case POPULATE_ORIGIN_INTEGRATIONS:
            return state.setIn(["currentIntegrations"],
                List(action.payload));

        case POPULATE_ORIGIN_PUBLIC_KEYS:
            if (action.error) {
                return state.setIn(
                    ["ui", "current", "publicKeyListErrorMessage"],
                    action.error.message
                );
            } else {
                return state.setIn(["currentPublicKeys"], List(action.payload)).
                    setIn(
                        ["ui", "current", "publicKeyListErrorMessage"],
                        undefined
                    );
            }

        case actionTypes.SET_CURRENT_ORIGIN:
            if (action.error) {
                return state.set("current", OriginRecord()).
                    setIn(["ui", "current", "errorMessage"],
                    action.error.message).
                    setIn(["ui", "current", "loading"], false).
                    setIn(["ui", "current", "exists"], false);
            } else {
                return state.set("current", OriginRecord(action.payload)).
                    setIn(["ui", "current", "errorMessage"], undefined).
                    setIn(["ui", "current", "exists"], true).
                    setIn(["ui", "current", "loading"], false);
            }
        case SET_CURRENT_ORIGIN_CREATING_FLAG:
            return state.setIn(["ui", "current", "creating"], action.payload);

        case actionTypes.SET_CURRENT_ORIGIN_ADDING_PRIVATE_KEY:
            return state.setIn(["ui", "current", "addingPrivateKey"],
                action.payload);

        case actionTypes.SET_CURRENT_ORIGIN_ADDING_PUBLIC_KEY:
            return state.setIn(["ui", "current", "addingPublicKey"],
                action.payload);

        case actionTypes.SET_CURRENT_ORIGIN_LOADING:
            return state.setIn(["ui", "current", "loading"],
                action.payload);

        case SET_ORIGIN_PRIVATE_KEY_UPLOAD_ERROR_MESSAGE:
            return state.setIn(["ui", "current", "privateKeyErrorMessage"],
                action.payload);

        case SET_ORIGIN_PUBLIC_KEY_UPLOAD_ERROR_MESSAGE:
            return state.setIn(["ui", "current", "publicKeyErrorMessage"],
                action.payload);

        case actionTypes.SET_ORIGIN_INTEGRATION_SAVE_ERROR_MESSAGE:
            return state.setIn(["ui", "current", "integrationsSaveErrorMessage"],
                action.payload);

        case SET_ORIGIN_USER_INVITE_ERROR_MESSAGE:
            return state.setIn(["ui", "current", "userInviteErrorMessage"],
                action.payload);

        case actionTypes.TOGGLE_ORIGIN_PICKER:
            return state.setIn(["ui", "isPickerOpen"],
                !state.getIn(["ui", "isPickerOpen"]));

        default:
            return state;
    }
}