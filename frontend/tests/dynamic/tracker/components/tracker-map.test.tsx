/**
 * @jest-environment jsdom
 */

import { TrackerMap } from 'src/dynamic/tracker/components/tracker-map';

import React from 'react';

import '@testing-library/jest-dom';

import { render, screen } from '@testing-library/react';

test('test-TrackerMap', () => {
  render(<TrackerMap positions="" />);
  expect(screen.getByText("OpenStreetMap")).toBeInTheDocument();
});
