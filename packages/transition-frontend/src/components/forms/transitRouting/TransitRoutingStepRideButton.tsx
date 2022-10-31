/*
 * Copyright 2022, Polytechnique Montreal and contributors
 *
 * This file is licensed under the MIT License.
 * License text available at https://opensource.org/licenses/MIT
 */
import React from 'react';
import { withTranslation, WithTranslation } from 'react-i18next';

import { secondsToMinutes } from 'chaire-lib-common/lib/utils/DateTimeUtils';
import serviceLocator from 'chaire-lib-common/lib/utils/ServiceLocator';

import { TrRoutingBoardingStep, TrRoutingUnboardingStep } from 'chaire-lib-common/lib/api/TrRouting';

export interface TransitRoutingStepRideButtonProps extends WithTranslation {
    boardingStep: TrRoutingBoardingStep;
    alightingStep: TrRoutingUnboardingStep;
    stepIndex: number;
}

const TransitRoutingStepRideButton: React.FunctionComponent<TransitRoutingStepRideButtonProps> = (
    props: TransitRoutingStepRideButtonProps
) => {
    const pathAttribs = serviceLocator.collectionManager.get('paths').getById(props.boardingStep.pathUuid).properties;
    const lineAttribs = serviceLocator.collectionManager.get('lines').getById(pathAttribs.line_id).attributes;
    const boardingNodeAttribs = serviceLocator.collectionManager.get('nodes').getById(props.boardingStep.nodeUuid)
        .properties;
    const alightingNodeAttribs = serviceLocator.collectionManager.get('nodes').getById(props.alightingStep.nodeUuid)
        .properties;

    // TODO tahini: Original code was using the objects, they are not yet in typescript so we'll use the attributes for now
    /*
    const path = new Path(
        serviceLocator.collectionManager.get('paths').getById(boardingStep.pathUuid).properties,
        false,
        serviceLocator.collectionManager
    );
    const line = path.getLine();
    const boardingNode = new Node(
        serviceLocator.collectionManager.get('nodes').getById(boardingStep.nodeUuid).properties,
        false,
        serviceLocator.collectionManager
    );
    const aligthingNode = new Node(
        serviceLocator.collectionManager.get('nodes').getById(alightingStep.nodeUuid).properties,
        false,
        serviceLocator.collectionManager
    );
    */
    const travelTimeSeconds = props.alightingStep.arrivalTimeSeconds - props.boardingStep.departureTimeSeconds;
    const travelTimeMinutes = secondsToMinutes(travelTimeSeconds, Math.round);

    return (
        <React.Fragment key={`ride${props.stepIndex}`}>
            <li className={'_list _o50 _indent'} onClick={undefined} key="boardingNode">
                <span className="_list-group">
                    <span className="_list-element _strong">{boardingNodeAttribs.name}</span>
                </span>
                <span className="_list-group _flush-right _right">
                    <span
                        className="_list-element"
                        title={`${props.boardingStep.waitingTimeSeconds} ${props.t('main:secondAbbr')}.`}
                    >
                        {props.boardingStep.waitingTimeMinutes} {props.t('main:minuteAbbr')}. (
                        {props.t('transit:transitRouting:waiting')})
                    </span>
                </span>
            </li>
            <li className="_clear" key="clearer1"></li>
            <li className={'_list'} onClick={undefined} key="ride">
                <span className="_list-group _left">
                    <span
                        className="_list-element _circle-button"
                        style={{ backgroundColor: lineAttribs?.color }}
                    ></span>
                    <img
                        className="_list-element _icon-alone"
                        src={`/dist/images/icons/transit/modes/${lineAttribs?.mode}_white.svg`}
                        alt={props.t(`transit:transitLine:modes:${lineAttribs?.mode}`)}
                    />
                </span>
                <span className="_list-group _left">
                    <span className="_list-element _strong">{lineAttribs?.shortname}</span>
                </span>
                <span className="_list-group _left">
                    <span className="_list-element">{lineAttribs?.longname}</span>
                </span>
                <span className="_list-group">
                    <span className="_list-element _em">
                        {pathAttribs.name
                            ? pathAttribs.name +
                              ' (' +
                              props.t(`transit:transitPath:directions:${pathAttribs.direction}`) +
                              ')'
                            : props.t(`transit:transitPath:directions:${pathAttribs.direction}`)}
                    </span>
                </span>
                <span className="_list-group _flush-right _right">
                    <span className="_list-element _em" title={`${travelTimeSeconds} ${props.t('main:secondAbbr')}.`}>
                        {travelTimeMinutes} {props.t('main:minuteAbbr')}.
                    </span>
                </span>
                <span className="_list-group _right">
                    <span className="_list-element">{Math.round(props.alightingStep.inVehicleDistanceMeters)} m</span>
                </span>
            </li>
            <li className="_clear" key="clearer2"></li>
            <li className={'_list _o50 _indent'} onClick={undefined} key="aligthingNode">
                <span className="_list-group">
                    <span className="_list-element _strong">{alightingNodeAttribs.name}</span>
                </span>
            </li>
            <li className="_clear" key="clearer3"></li>
        </React.Fragment>
    );
};

export default withTranslation(['transit', 'main'])(TransitRoutingStepRideButton);
