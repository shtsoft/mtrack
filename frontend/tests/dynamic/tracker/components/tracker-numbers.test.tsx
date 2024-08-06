/**
 * @jest-environment jsdom
 */

import { TrackerNumbers } from 'src/dynamic/tracker/components/tracker-numbers';

import React from 'react';

import '@testing-library/jest-dom';

import { render, screen } from '@testing-library/react';

test('test-TrackerNumbers', () => {
  render(<TrackerNumbers positions="" />);
  expect(screen.getByTestId("numbers")).toBeInTheDocument();
});
