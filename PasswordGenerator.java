/*
 * author: Ubunzu
 * Created in 21 Feb 2023
 * 
 * Simple UI app to generate a random passwords
 */

import java.awt.*;
import java.awt.datatransfer.Clipboard;
import java.awt.datatransfer.StringSelection;
import java.awt.event.*;
import javax.swing.*;
import javax.swing.event.ChangeEvent;
import javax.swing.event.ChangeListener;

public class PasswordGenerator extends JFrame implements ActionListener {
    private JLabel passwordLabel;
    private JButton generateButton, copyButton;
    private JSlider passwordLengthSlider;

    public PasswordGenerator() {
        super("Password Generator");

        // Create and configure UI components
        Image iconImage = Toolkit.getDefaultToolkit().getImage("icons/reset-password.png");
        passwordLabel = new JLabel("Click 'Generate' to create a password");
        generateButton = new JButton("Generate");
        copyButton = new JButton("Copy to Clipboard");
        passwordLengthSlider = new JSlider(JSlider.HORIZONTAL, 6, 24, 12);
        passwordLengthSlider.setMajorTickSpacing(6);
        passwordLengthSlider.setMinorTickSpacing(1);
        passwordLengthSlider.setPaintTicks(true);
        passwordLengthSlider.setPaintLabels(true);

        // Set the layout and add components to the main panel
        setIconImage(iconImage);

        JPanel mainPanel = new JPanel(new GridBagLayout());
        GridBagConstraints c = new GridBagConstraints();
        c.gridx = 0;
        c.gridy = 0;
        c.insets = new Insets(5, 5, 5, 5);
        mainPanel.add(passwordLabel, c);
        c.gridy++;
        mainPanel.add(generateButton, c);
        c.gridx++;
        mainPanel.add(copyButton, c);
        c.gridx = 0;
        c.gridy++;
        c.gridwidth = 2;
        mainPanel.add(passwordLengthSlider, c);

        // Add event listeners to buttons and slider
        generateButton.addActionListener(this);
        copyButton.addActionListener(this);
        passwordLengthSlider.addChangeListener(new ChangeListener() {
            public void stateChanged(ChangeEvent e) {
                JSlider slider = (JSlider) e.getSource();
                if (!slider.getValueIsAdjusting()) {
                    int passwordLength = slider.getValue();
                    passwordLabel.setText(generatePassword(passwordLength));
                }
            }
        });

        // Add the main panel to the frame
        getContentPane().add(mainPanel);

        // Configure the frame and make it visible
        setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        setSize(400, 200);
        setVisible(true);
    }

    public void actionPerformed(ActionEvent e) {
        if (e.getSource() == generateButton) {
            int passwordLength = passwordLengthSlider.getValue();
            passwordLabel.setText(generatePassword(passwordLength));
        } else if (e.getSource() == copyButton) {
            StringSelection stringSelection = new StringSelection(passwordLabel.getText());
            Clipboard clipboard = Toolkit.getDefaultToolkit().getSystemClipboard();
            clipboard.setContents(stringSelection, null);
        }
    }

    private String generatePassword(int length) {
        // Generate a password using a combination of letters, numbers, and symbols
        String uppercaseLetters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        String lowercaseLetters = "abcdefghijklmnopqrstuvwxyz";
        String numbers = "0123456789";
        String symbols = "!@#$%^&*()_-+={[}]|\\:;\"'<,>.?/";

        String allCharacters = uppercaseLetters + lowercaseLetters + numbers + symbols;
        StringBuilder passwordBuilder = new StringBuilder();
        for (int i = 0; i < length; i++) {
            int index = (int) (Math.random() * allCharacters.length());
            passwordBuilder.append(allCharacters.charAt(index));
        }
        return passwordBuilder.toString();
    }

    public static void main(String[] args) {
        new PasswordGenerator();
    }
}
